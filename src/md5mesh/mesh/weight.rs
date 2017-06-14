//! A module for parsing the weights in the mesh{} section.

use md5mesh::*;

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
    consume_rest_of_line_if_comment >>
    many0!(consume_rest_of_line_if_comment) >>
    (Weight{
      index: weight_index as usize,
      joint: joint as usize,
      bias: bias,
      pos: pos,
    })
  )
);

/// Parse weights section of mesh {}
named!(pub parse_md5mesh_mesh_weights<&str, Vec<Weight>>,
  do_parse!(
    // Get num weights
    space >>
    tag!("numweights") >>
    space >>
    num_weights: parse_u32 >>
    consume_rest_of_line_if_comment >>
    many0!(consume_rest_of_line_if_comment) >>

    // Parse all weights
    weights: count!( parse_md5mesh_mesh_weight, num_weights as usize ) >>

    (weights)
  )
);
