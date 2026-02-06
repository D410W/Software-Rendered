use std::rc::Rc;
use std::num::NonZeroU32;

use winit::application::ApplicationHandler;
use winit::event_loop::{ActiveEventLoop, OwnedDisplayHandle};
use winit::event::WindowEvent;
use winit::window::{Window, WindowId};
use winit::keyboard::Key;

use crate::softrender::Renderer;

pub struct App {
  window: Option<Rc<Window>>,
  surface: Option<softbuffer::Surface<OwnedDisplayHandle, Rc<Window>>>,
  
  renderer: Renderer,
}

impl App {
  pub fn new() -> Self {
    App{
      window: None,
      surface: None,
      
      renderer: Renderer::new(),
    }
  }
}

impl ApplicationHandler for App {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    self.window = Some(
      Rc::new(event_loop.create_window(Window::default_attributes()).unwrap())
    );
    
    let context = softbuffer::Context::new(event_loop.owned_display_handle()).unwrap();
    
    self.surface = Some(
      softbuffer::Surface::new(&context, self.window.clone().unwrap()).unwrap()
    );
  }

  fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
    match event {
      WindowEvent::CloseRequested => {
        println!("The close button was pressed. Stopping...");
        
        let final_fps = self.renderer.frame_counter as f32 / self.renderer.program_start.elapsed().as_secs_f32();
        println!("average fps through execution: {}.", final_fps);
        
        event_loop.exit();
      },
      WindowEvent::Resized(new_size) => {
        let Some(ref mut surface) = self.surface else {
          eprintln!("Resized fired before Resumed or after Suspended");
          return;
        };
        surface.resize(
          NonZeroU32::new(new_size.width).unwrap(),
          NonZeroU32::new(new_size.height).unwrap(),
        ).unwrap();
      },
      WindowEvent::KeyboardInput{ device_id: _, event: key_event, is_synthetic: _ } => {
        if key_event.state == winit::event::ElementState::Pressed && let Key::Character(c) = key_event.logical_key {
          match c.as_bytes()[0] as char {
            'w' => { self.renderer.camera_info.position.z -= 0.1; }
            's' => { self.renderer.camera_info.position.z += 0.1; }
            'a' => { self.renderer.camera_info.position.x -= 0.1; }
            'd' => { self.renderer.camera_info.position.x += 0.1; }
            'q' => { self.renderer.camera_info.position.y -= 0.1; }
            'e' => { self.renderer.camera_info.position.y += 0.1; }
            'j' => { self.renderer.camera_info.rotation += 0.1; }
            'l' => { self.renderer.camera_info.rotation -= 0.1; }
            _ => (),
          }
        }
      }
      WindowEvent::RedrawRequested => {
        let Some(ref mut surface) = self.surface else {
          eprintln!("RedrawRequested fired before Resumed or after Suspended");
          return;
        };
        let window = self.window.as_ref().unwrap();
        
        self.renderer.redraw(surface, &window);
        
        window.request_redraw();
      }
      _ => (),
    }
  }
}
