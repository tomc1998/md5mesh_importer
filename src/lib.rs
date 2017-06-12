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

pub struct MD5Scene {
  pub joint_list: Vec<Joint>,
  pub shader_name: String,
  pub verts: Vec<Vert>
}
impl MD5Scene {
  pub fn new() -> MD5Scene {
    MD5Scene { 
      joint_list: Vec::new(),
      shader_name: "".to_owned(),
      verts: Vec::new(),
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

use nom::{digit, space};
use std::str::FromStr;

/// Parse a number to a u32. Integer overflows will result in an error.
named!( parse_u32<&str, u32>, map_res!( digit, u32::from_str));
/// Parse a number to a i32. Integer overflows will result in an error.
named!( parse_i32<&str, u32>, map_res!( digit, u32::from_str));

/// Parser for the md5mesh header
named!(
  parse_md5mesh_header<&str, MD5MeshHeader>, 
  do_parse!(

    // Version
    tag!("MD5Version") >>
    many0!(space) >>
    md5_version: parse_u32 >>
    take_until_and_consume_s!("\n") >>

    // Command line 
    tag!("commandline") >>
    many0!(space) >>
    command_line: delimited!(tag!("\""), take_until!("\""), tag!("\"")) >>
    take!(1) >>

    // Optional empty line in header (not sure if essential to spec)
    opt!(alt!(tag!("\n") | tag!("\r\n") | tag!("\r"))) >>

    // Num Joints
    tag!("numJoints") >>
    many0!(space) >>
    num_joints: parse_u32 >>
    take_until_and_consume_s!("\n") >>

    // Num Meshes
    tag!("numMeshes") >>
    many0!(space) >>
    num_meshes: parse_u32 >>
    take_until_and_consume_s!("\n") >>

  (MD5MeshHeader {
    md5_version: md5_version,
    command_line: command_line.to_owned(),
    num_joints: num_joints,
    num_meshes: num_meshes,
  })
)
);

pub fn load_mesh<P: AsRef<Path>>(path: P) -> MD5Scene {
  use std::io::prelude::*;
  use std::fs::File;

  let mut f = File::open(path).unwrap();
  let mut data = String::new();
  f.read_to_string(&mut data).unwrap();

  let header = parse_md5mesh_header(&data);
  println!("{:?}", header);

  return MD5Scene::new();
}
