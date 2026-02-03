use std::rc::Rc;

use winit::application::ApplicationHandler;
use winit::event_loop::{ActiveEventLoop, OwnedDisplayHandle};
use winit::event::WindowEvent;
use winit::window::{Window, WindowId};

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
        event_loop.exit();
      },
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
