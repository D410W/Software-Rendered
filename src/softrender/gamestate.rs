use winit::event::KeyEvent;

use super::renderer::{*};

/// A Generic struct that implements the basic 'update' and 'draw' game-logic methods.
/// It requires the most basic methods that the Game logic needs.
pub trait GameState: Sized {
  fn new(ctx: &mut Renderer) -> Self;

  fn update(&mut self, ctx: &mut Renderer);
  
  fn draw(&mut self, ctx: &mut Renderer);
  
  fn key_input(&mut self, key_input: KeyEvent);
  // fn mouse_movement(&mut self, );
  
  fn should_run(&mut self) -> bool;
}
