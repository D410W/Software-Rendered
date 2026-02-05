use crate::softrender::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Instance {
  pub model_index: usize,
  pub position: Vec3,
  pub rotation: f32, // yaw in radians
}

#[derive(Debug, Clone, Copy)]
pub enum CullingEnum {
  None,
  Front,
  Back,
}

#[derive(Debug, Clone, Copy)]
pub struct RenderConfig {
  pub face_culling: CullingEnum,
  pub depth_buffering: bool,
  pub anti_aliasing: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct CameraInfo {
  pub position: Vec3,
  pub rotation: f32, // yaw in radians
  pub render_config: RenderConfig,
}
