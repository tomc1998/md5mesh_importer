//! This module is for parsing the joints {} section of the .md5mesh file

use md5mesh::*;

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

/// Parser for md5mesh joints section
named!(
  pub parse_md5mesh_joints<&str, Vec<Joint> >,
  do_parse!(
    // Parse the start of the block
    tag!("joints {") >>
    consume_rest_of_line_if_comment >>
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
        consume_rest_of_line_if_comment >>
        many0!(consume_rest_of_line_if_comment) >>
        (Joint::new_and_calc_ori_w( joint_name, parent_ix as isize, &joint_pos, &joint_ori ))
      )
    ) >>
    // Parse closing }
    tag!("}") >>
    consume_rest_of_line_if_comment >>
    many0!(consume_rest_of_line_if_comment) >>
    (joint_vec)
  )
);
