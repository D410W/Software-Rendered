use winit::event_loop::OwnedDisplayHandle;
use winit::window::Window;

use std::rc::Rc;
use std::time::Instant;
use std::collections::VecDeque;

use crate::softrender::{CameraInfo, RenderConfig, CullingEnum,
                        Instance, UnifiedGeometryBuffer, Vertex,
                        Vec3, Vec2}; // structs
use crate::softrender::{edge_function, edge_function_raw, translate_to_screen}; // funcs

type SoftSurface = softbuffer::Surface<OwnedDisplayHandle, Rc<Window>>;
type SoftBuffer<'a> = softbuffer::Buffer<'a, OwnedDisplayHandle, Rc<Window>>;

pub struct Renderer {
  // fps tracking
  last_frame: Instant,
  fps_measurement: f32,
  frame_counter: u64,
  
  frametime_hist: VecDeque<f32>,
  
  // geometry
  ugb: UnifiedGeometryBuffer,
  depth_buffer: Vec<f32>,
}

impl Renderer {
  
  pub fn new() -> Self {
    let mut ugb = UnifiedGeometryBuffer::default();
    ugb.init();
    
    Renderer{
      last_frame: Instant::now(),
      fps_measurement: 0.0,
      frame_counter: 0,
      
      frametime_hist: VecDeque::<f32>::from(vec![16.6; 10]),
      
      ugb,
      depth_buffer: Vec::new(),
    }
  }
  
  fn update_fps(&mut self) {
    let new_frame = Instant::now();
    let elapsed = new_frame.duration_since(self.last_frame);
    self.last_frame = new_frame;
    
    let smoothing = 0.9;
    self.fps_measurement = (self.fps_measurement * smoothing) + (1.0 / elapsed.as_secs_f32() * (1.0 - smoothing));
    
    self.frame_counter += 1;
    self.frametime_hist.pop_front();
    self.frametime_hist.push_back(elapsed.as_secs_f32());
  }
  
  pub fn redraw(&mut self, surface: &mut SoftSurface, window: &Window) {
  
    self.update_fps();

    let mut buffer = surface.buffer_mut().unwrap();
    buffer.fill(20 | (20 << 8) | (20 << 16));
    self.depth_buffer.resize(buffer.width().get() as usize * buffer.height().get() as usize, 0.0);
    self.depth_buffer.fill(f32::INFINITY);
    
    for id in 0..10 {
      self.rasterize_model(&mut buffer,
        Instance{
          model_index: 0,
          position: Vec3{x: -7.5 + ((id%5)*3) as f32, y: -2.0, z: -3.0 - ((id/5)*3) as f32},
          rotation: 0.0,
        },
        CameraInfo{
          position: Vec3::from_u32(0, 0, 0),
          rotation: f32::sin(self.frame_counter as f32 / 30.0),
          render_config: RenderConfig{
            face_culling: CullingEnum::Back,
            depth_buffering: true,
            anti_aliasing: false,
          },
        },
      );
    }
    
    // let width = buffer.width().get();
    // for y in 0..buffer.height().get() {
    //   for x in 0..width {
    //     let color: u32 = ((-200.0 * self.depth_buffer[(y * width + x) as usize]) as u32).min(255);
    //     buffer[(y * width + x) as usize] = color | color << 8 | color << 16;
    //   }
    // }
    
    println!("{:?}, fps: {}, low: {}", window.inner_size(), self.fps_measurement, 1.0 / self.frametime_hist.iter().max_by( |a, b| a.partial_cmp(b).unwrap() ).unwrap());
    buffer.present().unwrap();
  }
  
  pub fn rasterize_model(&mut self, buffer: &mut SoftBuffer, instance_info: Instance, camera_info: CameraInfo) {
    let model = self.ugb.models[instance_info.model_index];
    // let draw_indices = &self.ugb.indices[model.index_start..(model.index_start + model.index_count)];
    
    let swidth = buffer.width().get() as usize;
    let sheight = buffer.height().get() as usize;
    let screen_size = (swidth, sheight);
    
    let (sin_theta, cos_theta) = (instance_info.rotation).sin_cos();
    let rot_x = Vec3 { x: cos_theta, y: 0.0, z: -sin_theta };
    let rot_y = Vec3 { x: 0.0,       y: 1.0, z: 0.0 };
    let rot_z = Vec3 { x: sin_theta, y: 0.0, z: cos_theta };
    
    let (sin_theta_cam, cos_theta_cam) = (-camera_info.rotation).sin_cos();
    let cam_rot_x = Vec3 { x: cos_theta_cam, y: 0.0, z: -sin_theta_cam };
    let cam_rot_y = Vec3 { x: 0.0,           y: 1.0, z: 0.0 };
    let cam_rot_z = Vec3 { x: sin_theta_cam, y: 0.0, z: cos_theta_cam };
    
    let final_rot_x = rot_x.on_new_basis(cam_rot_x, cam_rot_y, cam_rot_z);
    let final_rot_y = rot_y.on_new_basis(cam_rot_x, cam_rot_y, cam_rot_z);
    let final_rot_z = rot_z.on_new_basis(cam_rot_x, cam_rot_y, cam_rot_z);
    let final_rel_pos = (instance_info.position - camera_info.position)
                        .on_new_basis(cam_rot_x, cam_rot_y, cam_rot_z);

    for i in (0..model.index_count).step_by(3) {
      let idx0 = self.ugb.indices[model.index_start + i];
      let idx1 = self.ugb.indices[model.index_start + i+1];
      let idx2 = self.ugb.indices[model.index_start + i+2];
      
      let mut v0 = self.ugb.vertices[model.base_vertex + idx0];
      let mut v1 = self.ugb.vertices[model.base_vertex + idx1];
      let mut v2 = self.ugb.vertices[model.base_vertex + idx2];
      
      v0.pos = v0.pos.on_new_basis(final_rot_x, final_rot_y, final_rot_z) + final_rel_pos;
      v1.pos = v1.pos.on_new_basis(final_rot_x, final_rot_y, final_rot_z) + final_rel_pos;
      v2.pos = v2.pos.on_new_basis(final_rot_x, final_rot_y, final_rot_z) + final_rel_pos;
      
      for v in [&v0, &v1, &v2] {
        if v.pos.z > 0.0 { return }
      }
      // Rasterize triangle (v0, v1, v2)
      {
        let v0_2d = translate_to_screen(&v0.pos, &screen_size);
        let v1_2d = translate_to_screen(&v1.pos, &screen_size);
        let v2_2d = translate_to_screen(&v2.pos, &screen_size);
        
        self.render_triangle_2d(buffer, &screen_size, &camera_info, &v0_2d, &v1_2d, &v2_2d, &v0, &v1, &v2);
      }
      
    }
  }
  
  fn render_triangle_2d(&mut self, buffer: &mut SoftBuffer,
                        screen_size: &(usize, usize), camera_info: &CameraInfo, 
                        v0_2d: &Vec2, v1_2d: &Vec2, v2_2d: &Vec2,
                        v0: &Vertex,  v1: &Vertex,  v2: &Vertex)
  {
    let area = edge_function(v0_2d, v1_2d, v2_2d);
    let inv_area = 1.0 / area;
    match camera_info.render_config.face_culling {
      CullingEnum::Front => {
        if area >= 0.0 { return; }
      },
      CullingEnum::Back => {
        if area <= 0.0 { return; }
      },
      CullingEnum::None => (),
    }
    
    let mut min = (screen_size.0, screen_size.1);
    let mut max = (0, 0);
    
    for v in [v0_2d, v1_2d, v2_2d] {
      let ux = v.x as usize;
      let uy = v.y as usize;
      
      if ux < min.0 { min.0 = ux }
      if ux > max.0 { max.0 = ux }
      if uy < min.1 { min.1 = uy }
      if uy > max.1 { max.1 = uy }
    }
    
    min.0 = min.0.max(0);
    min.1 = min.1.max(0);
    max.0 = max.0.min(screen_size.0 - 1);
    max.1 = max.1.min(screen_size.1 - 1);
    
    let inv_z0 = 1.0 / v0.pos.z;
    let inv_z1 = 1.0 / v1.pos.z;
    let inv_z2 = 1.0 / v2.pos.z;
    
    let r = (v0.color.x.abs() * 255.0) as u32 % 256;
    let g = (v1.color.y.abs() * 255.0) as u32 % 256;
    let b = (v2.color.z.abs() * 255.0) as u32 % 256;
    
    let tri_color = b | g << 8 | r << 16;
    
    for sy in min.1..=max.1 {
      let mut row_idx = sy * screen_size.0 + min.0;
      for sx in min.0..=max.0 {
        // if sx > 0 && sy > 0 && sx < screen_size.0 && sy < screen_size.1 {
          let weight0 = edge_function_raw(v1_2d, v2_2d, sx as f32, sy as f32) * inv_area;
          let weight1 = edge_function_raw(v2_2d, v0_2d, sx as f32, sy as f32) * inv_area;
          let weight2 = 1.0 - weight0 - weight1;
          
          let is_inside_tri = weight0 >= 0.0 && weight1 >= 0.0 && weight2 >= 0.0;
          
          if is_inside_tri {
            if camera_info.render_config.depth_buffering {
              let z_dist = weight0 * inv_z0 + weight1 * inv_z1 + weight2 * inv_z2;
              
              if z_dist < self.depth_buffer[row_idx] {
                self.depth_buffer[row_idx] = z_dist;
                buffer[row_idx] = tri_color;
              }
            } else {
              buffer[row_idx] = tri_color;
            }
          }
        // } else {
        //   println!("x: {}, y: {}", sx, sy);
        // }
        row_idx += 1;
      }
    }
    
    // for v in [v0, v1, v2] {
    //   buffer[v.y as usize * screen_size.0 + v.x as usize] = 0 | 0 << 8 | 255 << 16;
    // }
  }
}
