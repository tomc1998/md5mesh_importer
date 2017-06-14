//! This is a module for parsing the header of .md5mesh files.

use md5mesh::*;

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

/// Parser for the md5mesh header
named!(
  pub parse_md5mesh_header<&str, MD5MeshHeader>, 
  do_parse!(

    // Version
    tag!("MD5Version") >>
    space >>
    md5_version: parse_u32 >>
    consume_rest_of_line_if_comment >>

    // Command line 
    tag!("commandline") >>
    space >>
    command_line: delimited!(tag!("\""), take_until!("\""), tag!("\"")) >>
    consume_rest_of_line_if_comment >>

    // Optional empty line in header (not sure if essential to spec)
    opt!(consume_rest_of_line_if_comment) >>

    // Num Joints
    tag!("numJoints") >>
    space >>
    num_joints: parse_u32 >>
    consume_rest_of_line_if_comment >>

    // Num Meshes
    tag!("numMeshes") >>
    space >>
    num_meshes: parse_u32 >>
    consume_rest_of_line_if_comment >>

    (MD5MeshHeader {
      md5_version: md5_version,
      command_line: command_line.to_owned(),
      num_joints: num_joints,
      num_meshes: num_meshes,
    })
));
