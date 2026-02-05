use crate::softrender::Vec3;

pub struct Instance {
  pub model_index: usize,
  pub position: Vec3,
  pub rotation: f32, // in radians
}
