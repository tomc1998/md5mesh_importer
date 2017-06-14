//! A module for parsing the vertices in the mesh{} section.

use md5mesh::mesh::*;

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
    consume_rest_of_line_if_comment >>
    many0!(consume_rest_of_line_if_comment) >>
    (Vert{
      index: vert_index as usize,
      uv: uv,
      weight_ix_start: start_weight as usize,
      weight_ix_count: weight_count as usize 
    })
  )
);

/// Parse verts section of mesh {}
named!(pub parse_md5mesh_mesh_verts<&str, Vec<Vert>>,
  do_parse!(
    // Get num verts
    opt!(space) >>
    tag!("numverts") >>
    space >>
    num_verts: parse_u32 >>
    consume_rest_of_line_if_comment >>
    many0!(consume_rest_of_line_if_comment) >>

    // Parse all verts
    verts: count!( parse_md5mesh_mesh_vert, num_verts as usize ) >>

    (verts)
  )
);
