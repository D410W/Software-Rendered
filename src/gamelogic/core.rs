use winit::event::KeyEvent;

use crate::softrender::{*};
use super::common_structs::{*};

pub struct PlaneGame {
  pub should_run: bool,
}

impl GameState for PlaneGame {
  fn new(renderer: &mut Renderer) -> Self {
    let dennis_tex = renderer.load_texture_dds("src/dennis2.dds").unwrap();
    
    let _ = renderer.load_model_obj("src/monke.obj").unwrap();
    let dennis_id = renderer.load_textured_model_obj("src/dennis.obj", dennis_tex).unwrap();
    let smonke_model_id = renderer.load_model_obj("src/monke_smooth.obj").unwrap();
    renderer.remove_model(dennis_id);
    let cube_id = renderer.load_model_obj("src/untitled.obj").unwrap();
    renderer.remove_model(cube_id);
    
    println!("smonke: {smonke_model_id}, cube: {cube_id}");
    
    renderer.instances.push(
      Instance{
        model_id: smonke_model_id,
        position: Vec3{x: 0.0, y: 0.0, z: -3.0},
        rotation: Vec3{x: 0.0, y: 0.0, z: 0.0},
      }
    );
    
    PlaneGame{
      should_run: true,
    }
  }

  fn update(&mut self, _renderer: &mut Renderer) {
    
  }
  
  fn draw(&mut self, _renderer: &mut Renderer) {
    
  }
  
  fn key_input(&mut self, renderer: &mut Renderer, key_event: KeyEvent) {
    use winit::event::{ElementState, KeyEvent};
    use winit::keyboard::Key;
    
    if key_event.state == ElementState::Pressed && let Key::Character(c) = key_event.logical_key {
      match c.as_bytes()[0] as char {
        'w' => { renderer.camera_info.position.z -= 0.1; }
        's' => { renderer.camera_info.position.z += 0.1; }
        'a' => { renderer.camera_info.position.x -= 0.1; }
        'd' => { renderer.camera_info.position.x += 0.1; }
        'q' => { renderer.camera_info.position.y -= 0.1; }
        'e' => { renderer.camera_info.position.y += 0.1; }
        'j' => { renderer.camera_info.rotation.y += 0.1; }
        'l' => { renderer.camera_info.rotation.y -= 0.1; }
        'i' => { renderer.camera_info.rotation.x += 0.1; }
        'k' => { renderer.camera_info.rotation.x -= 0.1; }
        _ => (),
      }
    }
  }
  
  fn should_run(&mut self, _: &mut Renderer) -> bool {
    self.should_run
  }
  
  fn exit(&mut self, _: &mut Renderer) { }
  
}
