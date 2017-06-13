#[macro_use]
extern crate nom;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    use load_mesh;
    load_mesh("test.md5mesh");
  }
}

use std::path::Path;

/// A joint, containing a name, parent index, position and orientation.
#[derive(Clone, Debug)]
pub struct Joint {
  /// The name of the joint.
  pub name: String,
  /// The index of the parent joint. Joints will typically be stored in a
  /// `Vec`, so this will be an index into this array.
  pub parent_ix: isize,
  /// A vector representing this joint's position.
  pub position: [f32; 3],
  /// A quaternion representing this joint's orientation.
  pub orientation: [f32; 4],
}
impl Joint {
  /// Create a new join, and calculate the w value of the joint's orientation.
  pub fn new_and_calc_ori_w(name: &str, 
                            parent_ix: isize, 
                            position: &[f32; 3], 
                            ori: &[f32; 3]) -> Joint {
    let mut j = Joint {
      name: name.to_owned(),
      parent_ix: parent_ix,
      position: *position,
      orientation:[ori[0], ori[1], ori[2], 0.0],
    };
    j.calc_ori_w();
    return j;
  }

  /// Calculate the w value of the joint's orientation.
  pub fn calc_ori_w(&mut self) {
    let t = 1.0f32 - (self.orientation[0].powi(2)) - (self.orientation[1].powi(2)) - (self.orientation[2].powi(2));
    if t < 0.0 { self.orientation[3] = 0.0; }
    else { self.orientation[3] = -t.sqrt(); }
  }
}

/// A vertex in the MD5 file.
#[derive(Debug, Clone)]
pub struct Vert {
  /// Index of this vertex (for indexed rendering)
  index: usize,
  /// Texture coords
  uv: [f32; 2],
  /// Index of the first vertex weight
  weight_ix_start: usize,
  /// Number of weights applied to this vertex
  weight_ix_count: usize,
}

/// A tri in the MD5 file.
#[derive(Debug, Clone)]
pub struct Tri {
  /// Index of this tri
  index: usize,
  /// The first vert of the tri
  vert_index_1: usize,
  /// The second vert of the tri
  vert_index_2: usize,
  /// The third vert of the tri
  vert_index_3: usize,
}

/// A weight in the MD5 file.
#[derive(Debug, Clone)]
pub struct Weight {
  /// Index of this weight
  index: usize,
  /// The joint associated with this weight
  joint: usize,
  /// The bias of the weight (scale between 0.0 and 1.0, how much this weight
  /// contributes to the final vertex position)
  bias: f32,
  /// The position of this weight
  pos: [f32; 3],
}

#[derive(Debug, Clone)]
pub struct MD5Mesh {
  /// Name of the material
  shader: String,
  /// List of this mesh's vertices
  verts: Vec<Vert>,
  /// List of this mesh's vertices
  tris: Vec<Tri>,
  /// List of this mesh's weights
  weights: Vec<Weight>,
}

#[derive(Debug, Clone)]
pub struct MD5Scene {
  pub header: MD5MeshHeader,
  pub joint_list: Vec<Joint>,
  pub mesh_list: Vec<MD5Mesh>,
}
impl MD5Scene {
  pub fn new(header: MD5MeshHeader, joint_list: Vec<Joint>, mesh_list: Vec<MD5Mesh>) -> MD5Scene {
    MD5Scene { 
      header: header,
      joint_list: joint_list,
      mesh_list: mesh_list,
    }
  }
}

/// Header to the .md5mesh file.
#[derive(Debug, Clone)]
pub struct MD5MeshHeader {
  /// The MD5 version
  md5_version: u32,
  /// Not sure what this is. I assume a string indicating the command to run for a shell, like cmd.exe.
  command_line: String,
  /// Number of joints in the file (bones).
  num_joints: u32,
  /// Number of meshes in the file.
  num_meshes: u32,
}

use nom::{float_s, digit, space};
use std::str::FromStr;

/// Parse a number to a u32. Integer overflows will result in an error.
named!(parse_u32<&str, u32>, map_res!( digit, u32::from_str));
/// Parse a number to a i32. Integer overflows will result in an error.
named!(parse_i32<&str, i32>, 
  do_parse!(
    neg: opt!(tag!("-")) >>
    val: map_res!( digit, i32::from_str) >>
    (if neg.is_some() { -val } else { val })
  )
);
named!(parse_f32<&str, f32>, 
  do_parse!(
    neg: opt!(tag!("-")) >>
    val: call!(float_s) >>
    (if neg.is_some() { -val } else { val })
  )
);

/// Parser for the md5mesh header
named!(
  parse_md5mesh_header<&str, MD5MeshHeader>, 
  do_parse!(

    // Version
    tag!("MD5Version") >>
    space >>
    md5_version: parse_u32 >>
    take_until_and_consume_s!("\n") >>

    // Command line 
    tag!("commandline") >>
    space >>
    command_line: delimited!(tag!("\""), take_until!("\""), tag!("\"")) >>
    take!(1) >>

    // Optional empty line in header (not sure if essential to spec)
    opt!(alt!(tag!("\n") | tag!("\r\n") | tag!("\r"))) >>

    // Num Joints
    tag!("numJoints") >>
    space >>
    num_joints: parse_u32 >>
  take_until_and_consume_s!("\n") >>

  // Num Meshes
  tag!("numMeshes") >>
  space >>
  num_meshes: parse_u32 >>
  take_until_and_consume_s!("\n") >>

  (MD5MeshHeader {
    md5_version: md5_version,
    command_line: command_line.to_owned(),
    num_joints: num_joints,
    num_meshes: num_meshes,
  })
));

/// Parser for a 3 dimension vector of f32s (Position and orientation of joint).
named!(parse_vec3f<&str, [f32; 3]>,
       do_parse!(
         opt!(space) >>
         x: parse_f32 >>
         space >>
         y: parse_f32 >>
         space >>
         z: parse_f32 >>
         opt!(space) >>
         ([x, y, z])
     )
  );

/// Parser for a 2 dimension vector of f32s (UV coords).
named!(parse_vec2f<&str, [f32; 2]>,
       do_parse!(
         opt!(space) >>
         x: parse_f32 >>
         space >>
         y: parse_f32 >>
         opt!(space) >>
         ([x, y])
     )
  );

/// Parser for md5mesh joints section
named!(
  parse_md5mesh_joints<&str, Vec<Joint> >,
  do_parse!(
    // Parse the start of the block
    tag!("joints {") >>
    alt!(tag!("\n") | tag!("\r\n") | tag!("\r")) >>
    // Parse actual joints
    joint_vec: many0!(
      do_parse!(
        space >>
        joint_name: delimited!(tag!("\""), take_until!("\""), tag!("\"")) >>
        space >>
        parent_ix: parse_i32 >>
        space >>
        joint_pos: delimited!(tag!("("), parse_vec3f, tag!(")")) >>
        space >>
        joint_ori: delimited!(tag!("("), parse_vec3f, tag!(")")) >>
        opt!(space) >>
        many1!(alt!(tag!("\n") | tag!("\r\n") | tag!("\r"))) >>
        (Joint::new_and_calc_ori_w( joint_name, parent_ix as isize, &joint_pos, &joint_ori ))
      )
    ) >>
    // Parse closing }
    tag!("}") >>
    opt!(alt!(tag!("\n") | tag!("\r\n") | tag!("\r"))) >>
    (joint_vec)
  )
);

/// Consumes a line and parses a vert
named!(parse_md5mesh_mesh_vert<&str, Vert>,
  do_parse!(
    space >> 
    tag!("vert") >>
    space >> 
    vert_index: parse_u32 >>
    space >>
    uv: delimited!(tag!("("), parse_vec2f, tag!(")")) >>
    space >>
    start_weight: parse_u32 >> 
    space >>
    weight_count: parse_u32 >> 
    opt!(alt!(tag!("\n") | tag!("\r\n") | tag!("\r"))) >>
    (Vert{
      index: vert_index as usize,
      uv: uv,
      weight_ix_start: start_weight as usize,
      weight_ix_count: weight_count as usize 
    })
  )
);

/// Parse verts section of mesh {}
named!(parse_md5mesh_mesh_verts<&str, Vec<Vert>>,
  do_parse!(
    // Get num verts
    space >>
    tag!("numverts") >>
    space >>
    num_verts: parse_u32 >>
    alt!(tag!("\n") | tag!("\r\n") | tag!("\r")) >>

    // Parse all verts
    verts: count!( parse_md5mesh_mesh_vert, num_verts as usize ) >>

    (verts)
  )
);

/// Consumes a line and parses a tri
named!(parse_md5mesh_mesh_tri<&str, Tri>,
  do_parse!(
    opt!(space) >> 
    tag!("tri") >>
    space >> 
    tri_index: parse_u32 >>
    space >>
    v1: parse_u32 >>
    space >>
    v2: parse_u32 >>
    space >>
    v3: parse_u32 >>
    opt!(space) >>
    opt!(alt!(tag!("\n") | tag!("\r\n") | tag!("\r"))) >>
    (Tri{
      index: tri_index as usize,
      vert_index_1: v1 as usize,
      vert_index_2: v2 as usize,
      vert_index_3: v3 as usize,
    })
  )
);

/// Parse tris section of mesh {}
named!(parse_md5mesh_mesh_tris<&str, Vec<Tri>>,
  do_parse!(
    // Get num tris
    space >>
    tag!("numtris") >>
    space >>
    num_tris: parse_u32 >>
    alt!(tag!("\n") | tag!("\r\n") | tag!("\r")) >>

    // Parse all tris
    tris: count!( parse_md5mesh_mesh_tri, num_tris as usize ) >>

    (tris)
  )
);

/// Consumes a line and parses a weight
named!(parse_md5mesh_mesh_weight<&str, Weight>,
  do_parse!(
    opt!(space) >> 
    tag!("weight") >>
    space >> 
    weight_index: parse_u32 >>
    space >>
    joint: parse_u32 >>
    space >>
    bias: parse_f32 >>
    space >>
    pos: delimited!(tag!("("), parse_vec3f, tag!(")")) >>
    opt!(space) >>
    opt!(alt!(tag!("\n") | tag!("\r\n") | tag!("\r"))) >>
    (Weight{
      index: weight_index as usize,
      joint: joint as usize,
      bias: bias,
      pos: pos,
    })
  )
);

/// Parse weights section of mesh {}
named!(parse_md5mesh_mesh_weights<&str, Vec<Weight>>,
  do_parse!(
    // Get num weights
    space >>
    tag!("numweights") >>
    space >>
    num_weights: parse_u32 >>
    alt!(tag!("\n") | tag!("\r\n") | tag!("\r")) >>

    // Parse all weights
    weights: count!( parse_md5mesh_mesh_weight, num_weights as usize ) >>

    (weights)
  )
);

/// Consumes a line, parses the shader name
named!(parse_md5mesh_mesh_shader<&str, &str>,
  do_parse!(
    space >>
    tag!("shader") >>
    space >>
    shader_name: delimited!(tag!("\""), take_until!("\""), tag!("\"")) >>
    alt!(tag!("\n") | tag!("\r\n") | tag!("\r")) >>
    (shader_name)
  )
);

/// Parse mesh {} section of md5mesh
named!(parse_md5mesh_mesh<&str, MD5Mesh>,
  do_parse!(
    // Parse the start of the block
    tag!("mesh {") >>
    alt!(tag!("\n") | tag!("\r\n") | tag!("\r")) >>
    shader: parse_md5mesh_mesh_shader >>
    many0!(alt!(tag!("\n") | tag!("\r\n") | tag!("\r"))) >>
    verts: parse_md5mesh_mesh_verts >>
    many0!(alt!(tag!("\n") | tag!("\r\n") | tag!("\r"))) >>
    tris: parse_md5mesh_mesh_tris >>
    many0!(alt!(tag!("\n") | tag!("\r\n") | tag!("\r"))) >>
    weights: parse_md5mesh_mesh_weights >>
    many0!(alt!(tag!("\n") | tag!("\r\n") | tag!("\r"))) >>
    // Parse end of black
    tag!("}") >>
    (MD5Mesh{ verts: verts, tris: tris, weights: weights, shader: shader.to_owned() })
  )
);

/// Parser for the whole md5mesh file
named!(parse_md5mesh<&str, MD5Scene>,
  do_parse!(
    header: parse_md5mesh_header >>
    opt!(alt!(tag!("\n") | tag!("\r\n") | tag!("\r"))) >>
    joints: parse_md5mesh_joints >> 
    opt!(alt!(tag!("\n") | tag!("\r\n") | tag!("\r"))) >>
    meshes: many0!(
      do_parse!(
        mesh: parse_md5mesh_mesh >>
        many0!(alt!(tag!("\n") | tag!("\r\n") | tag!("\r"))) >>
        (mesh)
      )
    ) >>
    (MD5Scene::new(header, joints, meshes))
  )
);

pub fn load_mesh<P: AsRef<Path>>(path: P) -> MD5Scene {
  use std::io::prelude::*;
  use std::fs::File;

  let mut f = File::open(path).unwrap();
  let mut data = String::new();
  f.read_to_string(&mut data).unwrap();

  let scene = parse_md5mesh(&data);
  println!("{:?}", scene);

  return scene.unwrap().1;
}
