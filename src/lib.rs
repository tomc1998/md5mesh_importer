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
pub struct Vert {
}

pub struct MD5Mesh {
}

#[derive(Debug, Clone)]
pub struct MD5Scene {
  pub header: MD5MeshHeader,
  pub joint_list: Vec<Joint>,
  //pub shader_name: String,
  //pub verts: Vec<Vert>
}
impl MD5Scene {
  pub fn new(header: MD5MeshHeader, joint_list: Vec<Joint>) -> MD5Scene {
    MD5Scene { 
      header: header,
      joint_list: joint_list,
      //shader_name: "".to_owned(),
      //verts: Vec::new(),
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
    //tag!("}") >>
    opt!(alt!(tag!("\n") | tag!("\r\n") | tag!("\r"))) >>
    (joint_vec)
  )
);


/// Parser for the whole md5mesh file
named!(parse_md5mesh<&str, MD5Scene>,
  do_parse!(
    header: parse_md5mesh_header >>
    opt!(alt!(tag!("\n") | tag!("\r\n") | tag!("\r"))) >>
    joints: parse_md5mesh_joints >> 
    (MD5Scene::new(header, joints))
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
