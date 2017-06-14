//! This is a module for parsing the mesh{} section of the .md5mesh files.

use md5mesh::*;
use parse_vec2f;

mod vert;
mod tri;
mod weight;

use self::vert::*;
use self::tri::*;
use self::weight::*;
pub use self::vert::Vert;
pub use self::tri::Tri;
pub use self::weight::Weight;

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

/// Consumes a line, parses the shader name
named!(parse_md5mesh_mesh_shader<&str, &str>,
  do_parse!(
    space >>
    tag!("shader") >>
    space >>
    shader_name: delimited!(tag!("\""), take_until!("\""), tag!("\"")) >>
    consume_rest_of_line_if_comment >>
    many0!(consume_rest_of_line_if_comment) >>
    (shader_name)
  )
);

/// Parse mesh {} section of md5mesh
named!(pub parse_md5mesh_mesh<&str, MD5Mesh>,
  do_parse!(
    // Parse the start of the block
    tag!("mesh {") >>
    consume_rest_of_line_if_comment >>
    many0!(consume_rest_of_line_if_comment) >>
    shader: parse_md5mesh_mesh_shader >>
    many0!(consume_rest_of_line_if_comment) >>
    verts: parse_md5mesh_mesh_verts >>
    many0!(consume_rest_of_line_if_comment) >>
    tris: parse_md5mesh_mesh_tris >>
    many0!(consume_rest_of_line_if_comment) >>
    weights: parse_md5mesh_mesh_weights >>
    many0!(consume_rest_of_line_if_comment) >>
    // Parse end of black
    tag!("}") >>
    (MD5Mesh{ verts: verts, tris: tris, weights: weights, shader: shader.to_owned() })
  )
);


