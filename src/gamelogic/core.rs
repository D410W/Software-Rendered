use winit::event::KeyEvent;

use crate::softrender::{*};
use super::common_structs::{*};

pub struct PlaneGame {
  pub should_run: bool,
}

impl GameState for PlaneGame {
  fn new(ctx: &mut Renderer) -> Self {
    PlaneGame{
      should_run: false,
    }
  }

  fn update(&mut self, ctx: &mut Renderer) {
  
  }
  
  fn draw(&mut self, ctx: &mut Renderer) {
  
  }
  
  fn key_input(&mut self, key_input: KeyEvent) {
  
  }
  
  fn should_run(&mut self) -> bool {
    self.should_run
  }
}
