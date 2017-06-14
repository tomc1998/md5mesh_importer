//! A module for parsing the tris in the mesh{} section.

use md5mesh::*;

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
    consume_rest_of_line_if_comment >>
    many0!(consume_rest_of_line_if_comment) >>
    (Tri{
      index: tri_index as usize,
      vert_index_1: v1 as usize,
      vert_index_2: v2 as usize,
      vert_index_3: v3 as usize,
    })
  )
);

/// Parse tris section of mesh {}
named!(pub parse_md5mesh_mesh_tris<&str, Vec<Tri>>,
  do_parse!(
    // Get num tris
    space >>
    tag!("numtris") >>
    space >>
    num_tris: parse_u32 >>
    consume_rest_of_line_if_comment >>
    many0!(consume_rest_of_line_if_comment) >>

    // Parse all tris
    tris: count!( parse_md5mesh_mesh_tri, num_tris as usize ) >>

    (tris)
  )
);

