#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
  pub x: f32,
  pub y: f32,
}

#[allow(unused)]
impl Vec2 {
  pub fn from_u32(x: u32, y: u32) -> Self {
    Vec2{ x: x as f32, y: y as f32 }
  }
  pub fn from_usize(x: usize, y: usize) -> Self {
    Vec2{ x: x as f32, y: y as f32 }
  }
  
  pub fn dot(&self, rhs: Vec2) -> f32 {
    self.x * rhs.x + self.y * rhs.y
  }
  pub fn orthogonal(&self) -> Vec2 {
    Vec2{ x: -self.y, y: self.x }
  }
}

impl std::ops::Add<Vec2> for Vec2 { type Output = Vec2;
  fn add(self, rhs: Vec2) -> Vec2 { Vec2{ x: self.x + rhs.x, y: self.y + rhs.y} }
}
impl std::ops::Sub<Vec2> for Vec2 { type Output = Vec2;
  fn sub(self, rhs: Vec2) -> Vec2 { Vec2{ x: self.x - rhs.x, y: self.y - rhs.y} }
}
impl std::ops::Div<f32> for Vec2 { type Output = Vec2;
  fn div(self, rhs: f32) -> Vec2 { Vec2{ x: self.x / rhs, y: self.y / rhs} }
}
impl std::ops::Mul<f32> for Vec2 { type Output = Vec2;
  fn mul(self, rhs: f32) -> Vec2 { Vec2{ x: self.x * rhs, y: self.y * rhs} }
}

impl std::ops::Add<Vec2> for &Vec2 { type Output = Vec2;
  fn add(self, rhs: Vec2) -> Vec2 { Vec2{ x: self.x + rhs.x, y: self.y + rhs.y} }
}
impl std::ops::Sub<Vec2> for &Vec2 { type Output = Vec2;
  fn sub(self, rhs: Vec2) -> Vec2 { Vec2{ x: self.x - rhs.x, y: self.y - rhs.y} }
}
impl std::ops::Div<f32> for &Vec2 { type Output = Vec2;
  fn div(self, rhs: f32) -> Vec2 { Vec2{ x: self.x / rhs, y: self.y / rhs} }
}
impl std::ops::Mul<f32> for &Vec2 { type Output = Vec2;
  fn mul(self, rhs: f32) -> Vec2 { Vec2{ x: self.x * rhs, y: self.y * rhs} }
}

impl std::ops::Add<&Vec2> for &Vec2 { type Output = Vec2;
  fn add(self, rhs: &Vec2) -> Vec2 { Vec2{ x: self.x + rhs.x, y: self.y + rhs.y} }
}
impl std::ops::Sub<&Vec2> for &Vec2 { type Output = Vec2;
  fn sub(self, rhs: &Vec2) -> Vec2 { Vec2{ x: self.x - rhs.x, y: self.y - rhs.y} }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

#[allow(unused)]
impl Vec3 {
  pub fn from_u32(x: u32, y: u32, z: u32) -> Self {
    Vec3{ x: x as f32, y: y as f32, z: z as f32 }
  }
  pub fn from_usize(x: usize, y: usize, z: usize) -> Self {
    Vec3{ x: x as f32, y: y as f32, z: z as f32 }
  }
  
  pub fn on_new_basis(&self, bx: Vec3, by: Vec3, bz: Vec3) -> Vec3 {
    bx * self.x + by * self.y + bz * self.z
  }
}

impl std::ops::Add<Vec3> for Vec3 { type Output = Vec3;
  fn add(self, rhs: Vec3) -> Vec3 { Vec3{ x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z} }
}
impl std::ops::Sub<Vec3> for Vec3 { type Output = Vec3;
  fn sub(self, rhs: Vec3) -> Vec3 { Vec3{ x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z} }
}
impl std::ops::Mul<f32> for Vec3 { type Output = Vec3;
  fn mul(self, rhs: f32) -> Vec3 { Vec3{ x: self.x * rhs, y: self.y * rhs, z: self.z * rhs} }
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
  pub pos: Vec3,
  pub color: Vec3,
}
