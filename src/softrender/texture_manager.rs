#![allow(unused)]

use crate::softrender::{Color, Vec2};

pub struct TextureManager {
  pub indices_start: Vec<usize>,
  pub data: Vec<Color>,
  pub dimensions: Vec<(usize, usize)>,
}

impl TextureManager {
  pub fn new() -> Self {
    let mut tm = TextureManager{
      indices_start: Vec::new(),
      data: Vec::new(),
      dimensions: Vec::new(),
    };
    
    let bytes = [Color{ r: 255, g: 255, b: 255, a: 255 }; 1];
    tm.load_texture_color(&bytes, 1, 1);
    
    tm
  }

  #[inline(always)]  
  pub fn at(&self, texture_id: usize, uv: Vec2) -> Color {
    let width = self.dimensions[texture_id].0;
    let height = self.dimensions[texture_id].1;
    let x = (uv.x * (width - 1) as f32) as usize;
    let y = (uv.y * (height - 1) as f32) as usize;
    
    return self.data[self.indices_start[texture_id] + y * width + x];
  }
  #[inline(always)]
  pub fn at_raw(&self, texture_id: usize, x: usize, y: usize) -> Color {
    self.data[self.indices_start[texture_id] + y * self.dimensions[texture_id].0 + x]
  }
  
  pub fn load_texture_u32(&mut self, bytes: &[u32], width: usize, height: usize) -> usize {
    let tex_index = self.data.len();
    self.indices_start.push(tex_index);
    
    let color_slice: &[Color] = bytemuck::cast_slice(bytes);
    self.data.extend_from_slice(color_slice);
    
    self.dimensions.push((width, height));
    
    return tex_index;
  }
  pub fn load_texture_u32_vmirror(&mut self, bytes: &[u32], width: usize, height: usize) -> usize {
    let tex_index = self.data.len();
    self.indices_start.push(tex_index);
    
    for y in (0..height).rev() {
      let start = y * width;
      let end = start + width;
      let row = bytemuck::cast_slice(&bytes[start..end]);
      self.data.extend_from_slice(row);
    }
    
    self.dimensions.push((width, height));
    return tex_index;
  }
  pub fn load_texture_color(&mut self, bytes: &[Color], width: usize, height: usize) {
    self.indices_start.push(self.data.len());
    self.data.extend_from_slice(bytes);
    self.dimensions.push((width, height));
  }
}
