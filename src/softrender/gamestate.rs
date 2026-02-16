use winit::event::KeyEvent;

use super::renderer::{*};

/// A Generic struct that implements the basic 'update' and 'draw' game-logic methods.
/// It requires the most basic methods that the game logic needs.
pub trait GameState: Sized {
  fn new(renderer: &mut Renderer) -> Self;

  fn update(&mut self, renderer: &mut Renderer);
  
  fn draw(&mut self, renderer: &mut Renderer);
  
  fn key_input(&mut self, renderer: &mut Renderer, key_input: KeyEvent);
  // fn mouse_movement(&mut self, );
  
  fn should_run(&mut self, renderer: &mut Renderer) -> bool;
  
  fn exit(&mut self, renderer: &mut Renderer);
}
