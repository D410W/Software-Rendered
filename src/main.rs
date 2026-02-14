mod softrender;
mod gamelogic;
use winit::event_loop::{ControlFlow, EventLoop};

use gamelogic::core::PlaneGame;

fn main() {
  println!("Hello, CPU!");
  
  let event_loop = EventLoop::new().unwrap();
  event_loop.set_control_flow(ControlFlow::Poll);
  
  let mut app = softrender::App::<PlaneGame>::new();
  let _ = event_loop.run_app(&mut app);
}
