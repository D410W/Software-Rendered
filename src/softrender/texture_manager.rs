#![allow(unused)]

use crate::softrender::Color;

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
  
  pub fn load_texture_u32(&mut self, bytes: &[u32], width: usize, height: usize) {
    self.indices_start.push(self.data.len());
    
    self.data.reserve(bytes.len());
    for u32_color in bytes.iter().copied() {
      self.data.push(Color::from_u32(u32_color));
    }
    
    self.dimensions.push((width, height));
  }
  pub fn load_texture_color(&mut self, bytes: &[Color], width: usize, height: usize) {
    self.indices_start.push(self.data.len());
    self.data.extend_from_slice(bytes);
    self.dimensions.push((width, height));
  }
}
