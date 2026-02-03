mod softrender;
use winit::event_loop::{ControlFlow, EventLoop};

fn main() {
  println!("Hello, CPU!");
  
  let event_loop = EventLoop::new().unwrap();
  event_loop.set_control_flow(ControlFlow::Poll);
  
  let mut app = softrender::App::new();
  let _ = event_loop.run_app(&mut app);
}
