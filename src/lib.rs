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

pub struct MD5Scene {
  pub joint_list: Vec<Joint>,
}
impl MD5Scene {
  pub fn new() -> MD5Scene {
    MD5Scene { joint_list: Vec::new() }
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

pub fn load_mesh<P: AsRef<Path>>(path: P) -> MD5Scene {
  use std::io::prelude::*;
  use std::io::BufReader;
  use std::fs::File;

  let f = File::open(path).unwrap();
  let reader = BufReader::new(f);
  let lines : Vec<String> = reader.lines().map(|i| i.unwrap()).collect();

  let header = MD5MeshHeader {
    md5_version : lines[0].split_whitespace().skip(1).next().unwrap().parse().unwrap(),
    command_line : lines[1].split_whitespace().skip(1).next().unwrap().trim_matches('"').to_string(),
    num_joints : lines[3].split_whitespace().skip(1).next().unwrap().parse().unwrap(),
    num_meshes : lines[4].split_whitespace().skip(1).next().unwrap().parse().unwrap(),
  };

  // Parse out the joints and mesh blocks in the file
  let blocks : Vec<String> = lines[5..].join("\n").split("\n\n").map(|i| i.to_string()).collect();

  // Parse joints
  let mut joints_data : Vec<String> = blocks[0].trim().split("\n").map(|i| i.to_string()).collect();
  // Strip first and last item
  {
    let len = joints_data.len();
    joints_data.remove(len-1);
    joints_data.remove(0);
  }
  let mut joint_list : Vec<Joint> = Vec::with_capacity(header.num_joints as usize);
  for j in joints_data {
    let parts : Vec<String> = j.split_whitespace().map(|i| i.to_string()).collect();
    let mut joint = Joint {
      name: parts[0].trim_matches('"').to_string(),
      parent_ix: parts[1].parse().unwrap(),
      position: [ parts[3].parse().unwrap(), 
        parts[4].parse().unwrap(), 
        parts[5].parse().unwrap()
      ],
      orientation: [ parts[8].parse().unwrap(), 
        parts[9].parse().unwrap(), 
        parts[10].parse().unwrap(),
        0.0f32,
      ],
    };
    joint.calc_ori_w();
    joint_list.push(joint);
  }

  // Parse mesh

  return MD5Scene::new();
}
