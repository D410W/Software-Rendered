use winit::event_loop::OwnedDisplayHandle;
use winit::window::Window;

use std::rc::Rc;
use std::num::NonZeroU32;
use std::time::Instant;
use std::collections::VecDeque;

type SoftSurface = softbuffer::Surface<OwnedDisplayHandle, Rc<Window>>;

pub struct Renderer {
  last_frame: Instant,
  fps_measurement: f32,
  frame_counter: u64,
  
  frametime_hist: VecDeque<f32>,
}

impl Renderer {
  
  pub fn new() -> Self {
    Renderer{
      last_frame: Instant::now(),
      fps_measurement: 0.0,
      frame_counter: 0,
      
      frametime_hist: VecDeque::<f32>::with_capacity(10),
    }
  }
  
  fn update_fps(&mut self) {
    let new_frame = Instant::now();
    let elapsed = new_frame.duration_since(self.last_frame);
    self.last_frame = new_frame;
    
    let smoothing = 0.7;
    self.fps_measurement = (self.fps_measurement * smoothing) + (1.0 / elapsed.as_secs_f32() * (1.0 - smoothing));
    
    self.frame_counter += 1;
    self.frametime_hist.pop_front();
    self.frametime_hist.push_back(elapsed.as_secs_f32());
  }
  
  pub fn redraw(&mut self, surface: &mut SoftSurface, window: &Window) {
  
    self.update_fps();
    
  
    let size = window.inner_size();
    surface
      .resize(
        NonZeroU32::new(size.width).unwrap(),
        NonZeroU32::new(size.height).unwrap(),
      )
      .unwrap();

    let mut buffer = surface.buffer_mut().unwrap();
    for index in 0..(buffer.width().get() * buffer.height().get()) {
      let y = index / buffer.width().get();
      let x = index % buffer.width().get();
      let red = x % 255;
      let green = y % 255;
      let blue = (x*10 % (y+1)) % 255;

      buffer[index as usize] = blue | (green << 8) | (red << 16);
    }

    println!("{:?}, fps: {}, low: {}", size, self.fps_measurement,1.0 / self.frametime_hist.iter().min_by( |a, b| a.partial_cmp(b).unwrap() ).unwrap());
    buffer.present().unwrap();
  }
}



