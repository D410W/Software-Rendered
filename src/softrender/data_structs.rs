#![allow(unused)]

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
  pub x: f32,
  pub y: f32,
}

impl Vec2 {
  #[inline(always)]
  pub fn zero() -> Self {
    Vec2{ x: 0.0, y: 0.0 }
  }
  #[inline(always)]
  pub fn from_u32(x: u32, y: u32) -> Self {
    Vec2{ x: x as f32, y: y as f32 }
  }
  #[inline(always)]
  pub fn from_usize(x: usize, y: usize) -> Self {
    Vec2{ x: x as f32, y: y as f32 }
  }
  
  #[inline(always)]
  pub fn dot(&self, rhs: Vec2) -> f32 {
    self.x * rhs.x + self.y * rhs.y
  }
  #[inline(always)]
  pub fn orthogonal(&self) -> Vec2 {
    Vec2{ x: -self.y, y: self.x }
  }
}

impl std::ops::Add<Vec2> for Vec2 { type Output = Vec2;
  #[inline(always)]
  fn add(self, rhs: Vec2) -> Vec2 { Vec2{ x: self.x + rhs.x, y: self.y + rhs.y} }
}
impl std::ops::Sub<Vec2> for Vec2 { type Output = Vec2;
  #[inline(always)]
  fn sub(self, rhs: Vec2) -> Vec2 { Vec2{ x: self.x - rhs.x, y: self.y - rhs.y} }
}
impl std::ops::Div<f32> for Vec2 { type Output = Vec2;
  #[inline(always)]
  fn div(self, rhs: f32) -> Vec2 { Vec2{ x: self.x / rhs, y: self.y / rhs} }
}
impl std::ops::Mul<f32> for Vec2 { type Output = Vec2;
  #[inline(always)]
  fn mul(self, rhs: f32) -> Vec2 { Vec2{ x: self.x * rhs, y: self.y * rhs} }
}

impl std::ops::Add<Vec2> for &Vec2 { type Output = Vec2;
  #[inline(always)]
  fn add(self, rhs: Vec2) -> Vec2 { Vec2{ x: self.x + rhs.x, y: self.y + rhs.y} }
}
impl std::ops::Sub<Vec2> for &Vec2 { type Output = Vec2;
  #[inline(always)]
  fn sub(self, rhs: Vec2) -> Vec2 { Vec2{ x: self.x - rhs.x, y: self.y - rhs.y} }
}
impl std::ops::Div<f32> for &Vec2 { type Output = Vec2;
  #[inline(always)]
  fn div(self, rhs: f32) -> Vec2 { Vec2{ x: self.x / rhs, y: self.y / rhs} }
}
impl std::ops::Mul<f32> for &Vec2 { type Output = Vec2;
  #[inline(always)]
  fn mul(self, rhs: f32) -> Vec2 { Vec2{ x: self.x * rhs, y: self.y * rhs} }
}

impl std::ops::Add<&Vec2> for &Vec2 { type Output = Vec2;
  #[inline(always)]
  fn add(self, rhs: &Vec2) -> Vec2 { Vec2{ x: self.x + rhs.x, y: self.y + rhs.y} }
}
impl std::ops::Sub<&Vec2> for &Vec2 { type Output = Vec2;
  #[inline(always)]
  fn sub(self, rhs: &Vec2) -> Vec2 { Vec2{ x: self.x - rhs.x, y: self.y - rhs.y} }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Vec3 {
  #[inline(always)]
  pub fn from_u32(x: u32, y: u32, z: u32) -> Self {
    Vec3{ x: x as f32, y: y as f32, z: z as f32 }
  }
  #[inline(always)]
  pub fn from_usize(x: usize, y: usize, z: usize) -> Self {
    Vec3{ x: x as f32, y: y as f32, z: z as f32 }
  }
  
  #[inline(always)]
  pub fn on_new_basis(&self, bx: Vec3, by: Vec3, bz: Vec3) -> Vec3 {
    bx * self.x + by * self.y + bz * self.z
  }
}

impl std::ops::Add<Vec3> for Vec3 { type Output = Vec3;
  #[inline(always)]
  fn add(self, rhs: Vec3) -> Vec3 { Vec3{ x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z} }
}
impl std::ops::Sub<Vec3> for Vec3 { type Output = Vec3;
  #[inline(always)]
  fn sub(self, rhs: Vec3) -> Vec3 { Vec3{ x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z} }
}
impl std::ops::Mul<f32> for Vec3 { type Output = Vec3;
  #[inline(always)]
  fn mul(self, rhs: f32) -> Vec3 { Vec3{ x: self.x * rhs, y: self.y * rhs, z: self.z * rhs} }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Vec4 {
  pub w: f32,
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Vec4 {
  #[inline(always)]
  pub fn to_u32(&self) -> u32 {
    u32::from_le_bytes([self.x as u8, self.y as u8, self.z as u8, self.w as u8 ])
  }
  #[inline(always)]
  pub fn to_u32_safe(&self) -> u32 {
    u32::from_le_bytes([self.x.clamp(0.0, 255.0) as u8,
                        self.y.clamp(0.0, 255.0) as u8,
                        self.z.clamp(0.0, 255.0) as u8,
                        self.w.clamp(0.0, 255.0) as u8
    ])
  }
}

impl std::ops::Add<Vec4> for Vec4 { type Output = Vec4;
  #[inline(always)]
  fn add(self, rhs: Vec4) -> Vec4 { Vec4{ x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z, w: self.w + rhs.w} }
}
impl std::ops::AddAssign<Vec4> for Vec4 {
  #[inline(always)]
  fn add_assign(&mut self, rhs: Vec4) { self.x += rhs.x; self.y += rhs.y; self.z += rhs.z; self.w += rhs.w; }
}
impl std::ops::DivAssign<f32> for Vec4 {
  #[inline(always)]
  fn div_assign(&mut self, rhs: f32) { self.x /= rhs; self.y /= rhs; self.z /= rhs; self.w /= rhs; }
}
impl std::ops::Sub<Vec4> for Vec4 { type Output = Vec4;
  #[inline(always)]
  fn sub(self, rhs: Vec4) -> Vec4 { Vec4{ x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z, w: self.w + rhs.w} }
}
impl std::ops::Mul<f32> for Vec4 { type Output = Vec4;
  #[inline(always)]
  fn mul(self, rhs: f32) -> Vec4 { Vec4{ x: self.x * rhs, y: self.y * rhs, z: self.z * rhs, w: self.w * rhs} }
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
  pub pos: Vec3,
  pub uv: Vec2,
  pub color: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}

impl Color {
    // Fast conversion that compiles down to a simple move on Little Endian
  #[inline(always)]
  pub fn from_u32(val: u32) -> Self {
    let bytes = u32::to_le_bytes(val);
    Color{ r: bytes[0], g: bytes[1], b: bytes[2], a: bytes[3], }
  }
  
  #[inline(always)]
  pub fn to_u32(&self) -> u32 {
    u32::from_le_bytes([self.r, self.g, self.b, self.a])
  }
  #[inline(always)]
  pub fn to_vec4(&self) -> Vec4 {
    Vec4{ w: self.a as f32, x: self.r as f32, y: self.g as f32, z: self.b as f32}
  }
}
