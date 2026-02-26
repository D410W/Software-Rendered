use winit::event_loop::OwnedDisplayHandle;
use winit::window::Window;

use std::rc::Rc;
use std::io::Read;
use std::fs::File;
use std::time::Instant;
use std::collections::VecDeque;

use crate::softrender::{CameraInfo, RenderConfig, CullingEnum,
                        Instance, UnifiedGeometryBuffer, Vertex,
                        Vec4, Vec3, Vec2, TextureManager,
                        AttributeBundle}; // structs
use crate::softrender::{edge_function, edge_function_raw, translate_to_screen}; // funcs

type SoftSurface = softbuffer::Surface<OwnedDisplayHandle, Rc<Window>>;
type SoftBuffer<'a> = softbuffer::Buffer<'a, OwnedDisplayHandle, Rc<Window>>;

pub struct Renderer {
  // fps tracking
  pub program_start: Instant,
  last_frame: Instant,
  fps_measurement: f32,
  pub frame_counter: u64,
  
  frametime_hist: VecDeque<f32>,
  
  // geometry
  ugb: UnifiedGeometryBuffer,
  depth_buffer: Vec<f32>,
  downscaled_db: Vec<f32>,
  
  pub instances: Vec<Instance>,
  pub camera_info: CameraInfo,
  
  // texturing
  tm: TextureManager,
  
  // debug
  triangles_rendered: u32,
}

impl Renderer {
  
  pub fn new() -> Self {

    let tm = TextureManager::new();
    
    let ugb = UnifiedGeometryBuffer::default();
    
    Renderer{
      program_start: Instant::now(),
      last_frame: Instant::now(),
      fps_measurement: 0.0,
      frame_counter: 0,
      
      frametime_hist: VecDeque::<f32>::from(vec![0.0; 10]),
      
      ugb,
      depth_buffer: Vec::new(),
      downscaled_db: Vec::new(),
      instances: Vec::new(),
      camera_info: CameraInfo{
        position: Vec3{ x: 0.0, y: 0.0, z: 0.0 },
        rotation: Vec3{ x: 0.0, y: 0.0, z: 0.0 },
        render_config: RenderConfig{
          face_culling: CullingEnum::Back,
          debug_bounding_boxes: false,
          z_pyramid: true,
          affine_color: true,
        },
      },
      
      tm,
      
      triangles_rendered: 0,
    }
  }
  
  pub fn load_texture_dds(&mut self, file_path: impl AsRef<std::path::Path>) -> std::io::Result<usize> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Read failed");

    if &buffer[0..4] != b"DDS " { return Err(std::io::Error::other("Not a DDS file")); }

    let height = u32::from_le_bytes(buffer[12..16].try_into().unwrap()) as usize;
    let width = u32::from_le_bytes(buffer[16..20].try_into().unwrap()) as usize;
    // let pitch = u32::from_le_bytes(buffer[20..24].try_into().unwrap()) as usize;
    
    let pixel_data_start = 128;
    let raw_bytes = &buffer[pixel_data_start..];

    let u32_pixels: Vec<u32> = raw_bytes
      .chunks_exact(4)
      .map(|chunk| { u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]) })
      .collect();
    
    return Ok(self.tm.load_texture_u32_vmirror(u32_pixels.as_slice(), width, height));
  }
  pub fn load_model_obj(&mut self, file_path: impl AsRef<std::path::Path>) -> std::io::Result<usize> {
    self.ugb.load_model_obj(file_path)
  }
  pub fn load_textured_model_obj(&mut self, file_path: impl AsRef<std::path::Path>, texture_id: usize) -> std::io::Result<usize> {
    self.ugb.load_textured_model_obj(file_path, texture_id)
  }
  pub fn remove_model(&mut self, id: usize) -> bool {
    self.ugb.remove_model(id)
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
    self.triangles_rendered = 0;

    let mut buffer = surface.buffer_mut().unwrap();
    buffer.fill(20 | (20 << 8) | (20 << 16));
    
    self.depth_buffer.resize(buffer.len(), 0.0);
    self.depth_buffer.fill(f32::INFINITY);
    
    let downscaled_factor = 8;
    self.downscaled_db.resize(buffer.len() / usize::pow(downscaled_factor, 2), 0.0);
    self.downscaled_db.fill(0.0);
    
    // for id in 0..50 {
    //   self.rasterize_model(&mut buffer,
    //     Instance{
    //       model_index: 0,
    //       position: Vec3{x: -6.0 + ((id%5)*3) as f32, y: -2.0, z: -5.0 - ((id/5)*3) as f32},
    //       rotation: 0.0,
    //     },
    //     self.camera_info,
    //   );
    // }
    
    // {
    // self.rasterize_model(&mut buffer,
    //   Instance{
    //     model_index: 2,
    //     position: Vec3{ x: 0.0, y: 0.0, z: -2.0 },
    //     rotation: 0.0,
    //   },
    //   self.camera_info,
    // );
    
    // for id in 0..10 {
    //   self.rasterize_model(&mut buffer,
    //     Instance{
    //       model_index: 0,
    //       position: Vec3{ x: 0.0, y: 0.0, z: -4.0 - id as f32 },
    //       rotation: 0.0,
    //     },
    //     self.camera_info,
    //   );
    // }
    // }
    
    // self.rasterize_model(&mut buffer,
    //   Instance{
    //     model_index: 1,
    //     position: Vec3{x: 0.0, y: -90.0, z: -100.0},
    //     rotation: self.frame_counter as f32 / 100.0,
    //   },
    //   self.camera_info,
    // );
        
    let mut instances = self.instances.clone();
    instances.sort_by(|a, b| a.position.z.total_cmp(&b.position.z));
    for inst in instances.iter() {
      self.rasterize_model(&mut buffer, *inst, self.camera_info);
    }
    
    // let width = buffer.width().get();
    // for y in 0..buffer.height().get() {
    //   for x in 0..width {
    //     // let color: u32 = ((-200.0 * self.depth_buffer[(y * width + x) as usize]) as u32).min(255);
    //     // let color: u32 = ((-200.0 * self.downscaled_db[((y >> 3) * (width >> 3) + (x >> 3)) as usize]) as u32).min(255);
    //     // buffer[(y * width + x) as usize] = color | color << 8 | color << 16;
    //     buffer[(y * width + x) as usize] = self.tm.at_raw(1, x as usize, y as usize).to_u32();
    //   }
    // }
    
    println!("triangles rendered this frame: {}", self.triangles_rendered);
    println!("{:?}, fps: {}, low: {}", window.inner_size(), self.fps_measurement, 1.0 / self.frametime_hist.iter().max_by( |a, b| a.partial_cmp(b).unwrap() ).unwrap());
    buffer.present().unwrap();
  }
  
  pub fn rasterize_model(&mut self, buffer: &mut SoftBuffer, instance_info: Instance, camera_info: CameraInfo) {
    // let model = self.ugb.models[instance_info.model_index];
    let Some(model) = self.ugb.get_model(instance_info.model_id) else { return };
    
    let swidth = buffer.width().get() as usize;
    let sheight = buffer.height().get() as usize;
    let screen_size = (swidth, sheight);
    
    let (sin_x, cos_x) = (instance_info.rotation.x).sin_cos();
    let (sin_y, cos_y) = (instance_info.rotation.y).sin_cos();
    let (sin_z, cos_z) = (instance_info.rotation.z).sin_cos();
    let rot_x = Vec3 { x: cos_z*cos_y,                     y: sin_z*cos_y,                     z: -sin_y      };
    let rot_y = Vec3 { x: cos_z*sin_y*sin_x - sin_z*cos_x, y: sin_z*sin_y*sin_x + cos_z*cos_x, z: cos_y*sin_x };
    let rot_z = Vec3 { x: cos_z*sin_y*cos_x + sin_z*sin_x, y: sin_z*sin_y*cos_x - cos_z*sin_x, z: cos_y*cos_x };
    
    let (sin_x_cam, cos_x_cam) = (-camera_info.rotation.x).sin_cos();
    let (sin_y_cam, cos_y_cam) = (-camera_info.rotation.y).sin_cos();
    let (sin_z_cam, cos_z_cam) = (-camera_info.rotation.z).sin_cos();
    let cam_rot_x = Vec3 { x: cos_z_cam*cos_y_cam,                                 y: sin_z_cam*cos_y_cam,                                 z: -sin_y_cam          };
    let cam_rot_y = Vec3 { x: cos_z_cam*sin_y_cam*sin_x_cam - sin_z_cam*cos_x_cam, y: sin_z_cam*sin_y_cam*sin_x_cam + cos_z_cam*cos_x_cam, z: cos_y_cam*sin_x_cam };
    let cam_rot_z = Vec3 { x: cos_z_cam*sin_y_cam*cos_x_cam + sin_z_cam*sin_x_cam, y: sin_z_cam*sin_y_cam*cos_x_cam - cos_z_cam*sin_x_cam, z: cos_y_cam*cos_x_cam };
    
    let final_rot_x = rot_x.on_new_basis(cam_rot_x, cam_rot_y, cam_rot_z);
    let final_rot_y = rot_y.on_new_basis(cam_rot_x, cam_rot_y, cam_rot_z);
    let final_rot_z = rot_z.on_new_basis(cam_rot_x, cam_rot_y, cam_rot_z);
    let final_rel_pos = (instance_info.position - camera_info.position)
                        .on_new_basis(cam_rot_x, cam_rot_y, cam_rot_z);
    
    let mut model_bounding_min = Vec2{ x:  f32::INFINITY, y:  f32::INFINITY };
    let mut model_bounding_max = Vec2{ x: -f32::INFINITY, y: -f32::INFINITY };
    let mut closest_z = -f32::INFINITY;
    
    let mut is_visible: bool = {
      for i in 0..8 {
        let corner = Vec3{
          x: if i & 1 == 0 { model.min_extents.x } else { model.max_extents.x },
          y: if i & 2 == 0 { model.min_extents.y } else { model.max_extents.y },
          z: if i & 4 == 0 { model.min_extents.z } else { model.max_extents.z },
        }.on_new_basis(final_rot_x, final_rot_y, final_rot_z) + final_rel_pos;
        
        if corner.z >= 0.0 { continue; }
        
        let projected = translate_to_screen(&corner, &screen_size);
        
        model_bounding_min.x = model_bounding_min.x.min(projected.x);
        model_bounding_min.y = model_bounding_min.y.min(projected.y);
        model_bounding_max.x = model_bounding_max.x.max(projected.x);
        model_bounding_max.y = model_bounding_max.y.max(projected.y);
        closest_z = closest_z.max(corner.z);
      }
      
      if model_bounding_max.x > 0.0 &&
         model_bounding_max.y > 0.0 &&
         model_bounding_min.x < screen_size.0 as f32 &&
         model_bounding_min.y < screen_size.1 as f32 {
        true
      } else {
        false
      }
    };
    
    if !is_visible { return; }
    
    // culling through downscaled depth-buffer
    let pyramid_bounds = if camera_info.render_config.z_pyramid {
      closest_z = 1.0 / closest_z;
      // println!("{}", closest_z);
      is_visible = false;
      let start_y = (model_bounding_min.y as usize) >> 3;
      let end_y   = (model_bounding_max.y as usize) >> 3;
      let start_x = (model_bounding_min.x as usize) >> 3;
      let end_x   = (model_bounding_max.x as usize) >> 3;

      let max_tile_y = (screen_size.1 >> 3).saturating_sub(1);
      let max_tile_x = (screen_size.0 >> 3).saturating_sub(1);
      
      let sx = start_x.min(max_tile_x);
      let ex = end_x.min(max_tile_x);
      let sy = start_y.min(max_tile_y);
      let ey = end_y.min(max_tile_y);
      
      for ty in sy..=ey {
        for tx in sx..=ex {
          if closest_z < self.downscaled_db[ty * (swidth >> 3) + tx] {
            is_visible = true;
            break;
          }
        }
        if is_visible { break; }
      }
      
      if !is_visible { return; }
      
      Some((sx, ex, sy, ey))
    } else {
      None
    };
    
    let texture_size = self.tm.dimensions[model.texture_id];

    // triangles
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
      
      let mut should_skip = false;
      for v in [&v0, &v1, &v2] {
        if v.pos.z > 0.0 { should_skip = true; break; }
      }
      if should_skip { continue }
      // Rasterize triangle (v0, v1, v2)
      {
        let v0_2d = translate_to_screen(&v0.pos, &screen_size);
        let v1_2d = translate_to_screen(&v1.pos, &screen_size);
        let v2_2d = translate_to_screen(&v2.pos, &screen_size);
        
        self.render_triangle_2d(buffer, screen_size, &camera_info, model.texture_id, texture_size, &v0_2d, &v1_2d, &v2_2d, &v0, &v1, &v2);
      }
      
    }
    
    if camera_info.render_config.debug_bounding_boxes {
      for id in 0..4 {
        let p = (
          if id & 1 == 0 { model_bounding_min.x } else { model_bounding_max.x } as usize,
          if id & 2 == 0 { model_bounding_min.y } else { model_bounding_max.y } as usize
        );
        
        if p.0 >= swidth || p.1 >= sheight { continue }
        buffer[p.1 * swidth + p.0] = 0 | 255 << 8 | 0 << 16;
      }
    }
    
    // updating the downscaled_db
    if let Some((sx, ex, sy, ey)) = pyramid_bounds {
      for ty in sy..=ey {
        for tx in sx..=ex {
          // println!("{:?}", (tx, ty));
          self.update_downscaled_depth(screen_size.0, tx, ty);
        }
      }
    }
  }
  
  fn update_downscaled_depth(&mut self, screen_width: usize, x: usize, y: usize) {    
    let mut max_depth: f32 = 0.0; // self.downscaled_db[y * (width >> 3) + x];
    
    let start_x = x << 3;
    let start_y = y << 3;
    
    for y in start_y..(start_y + 8) {
      for x in start_x..(start_x + 8) {
        max_depth = max_depth.min(self.depth_buffer[y * screen_width + x]);
      }
    }
    
    self.downscaled_db[y * (screen_width >> 3) + x] = max_depth;
    
  }
  
  fn render_triangle_2d(&mut self, buffer: &mut SoftBuffer, screen_size: (usize, usize),
                        camera_info: &CameraInfo, texture_id: usize, texture_size: (usize, usize),
                        v0_2d: &Vec2, v1_2d: &Vec2, v2_2d: &Vec2,
                        v0: &Vertex,  v1: &Vertex,  v2: &Vertex)
  {
    let area = edge_function(v0_2d, v1_2d, v2_2d);
    if area.abs() < 1e-6 { return; }
    let inv_area = 1.0 / area;
    
    match camera_info.render_config.face_culling {
      CullingEnum::Both => { return; }
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
    
    // calculating bounding box's min and max
    for v in [v0_2d, v1_2d, v2_2d] {
      let uns_x = v.x.max(0.0) as usize;
      let uns_y = v.y.max(0.0) as usize;
      
      if uns_x < min.0 { min.0 = uns_x }
      if uns_x > max.0 { max.0 = uns_x }
      if uns_y < min.1 { min.1 = uns_y }
      if uns_y > max.1 { max.1 = uns_y }
    }
    
    min.0 = min.0.max(0);
    min.1 = min.1.max(0);
    max.0 = max.0.min(screen_size.0 - 1);
    max.1 = max.1.min(screen_size.1 - 1);
        
    if max.0 > 0 && max.1 > 0 && min.0 < screen_size.0 && min.1 < screen_size.1 {
      self.triangles_rendered += 1;
    } else {
      return;
    }
        
    // pre-calculating weights
    let step_x_w0 = v2_2d.y - v1_2d.y;
    let step_x_w1 = v0_2d.y - v2_2d.y;
    let step_x_w2 = v1_2d.y - v0_2d.y;
    
    let step_y_w0 = v1_2d.x - v2_2d.x;
    let step_y_w1 = v2_2d.x - v0_2d.x;
    let step_y_w2 = v0_2d.x - v1_2d.x;
    
    let w0_row = edge_function_raw(v1_2d, v2_2d, min.0 as f32 + 0.5, min.1 as f32 + 0.5);
    let w1_row = edge_function_raw(v2_2d, v0_2d, min.0 as f32 + 0.5, min.1 as f32 + 0.5);
    let w2_row = edge_function_raw(v0_2d, v1_2d, min.0 as f32 + 0.5, min.1 as f32 + 0.5);
    
    // z values
    let iz0 = 1.0 / v0.pos.z;
    let iz1 = 1.0 / v1.pos.z;
    let iz2 = 1.0 / v2.pos.z;
    let inv_z0 = iz0 * inv_area;
    let inv_z1 = iz1 * inv_area;
    let inv_z2 = iz2 * inv_area;
    
    // color values
    let mut inv_col0 = v0.color.to_vec4() * inv_area;
    let mut inv_col1 = v1.color.to_vec4() * inv_area;
    let mut inv_col2 = v2.color.to_vec4() * inv_area;
    
    if !camera_info.render_config.affine_color {
      inv_col0 = Vec4{ x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
      inv_col1 = Vec4{ x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
      inv_col2 = Vec4{ x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
    }
    
    // uv values
    let mut inv_uv0 = v0.uv * inv_z0; // or v0.uv * (1.0 / v0.pos.z) * inv_area
    inv_uv0.x *= texture_size.0 as f32;
    inv_uv0.y *= texture_size.1 as f32;
    
    let mut inv_uv1 = v1.uv * inv_z1;
    inv_uv1.x *= texture_size.0 as f32;
    inv_uv1.y *= texture_size.1 as f32;

    let mut inv_uv2 = v2.uv * inv_z2;
    inv_uv2.x *= texture_size.0 as f32;
    inv_uv2.y *= texture_size.1 as f32;
    
    let inv_normal0 = v0.normal * inv_area;
    let inv_normal1 = v1.normal * inv_area;
    let inv_normal2 = v2.normal * inv_area;
    
    let build_bundle = |a: f32, b: f32, c: f32| AttributeBundle{
      weight0: a, weight1: b, weight2: c,
      z: inv_z0 * a + inv_z1 * b + inv_z2 * c,
      color: inv_col0 * a + inv_col1 * b + inv_col2 * c,
      uv: inv_uv0 * a + inv_uv1 * b + inv_uv2 * c,
      normal: inv_normal0 * a + inv_normal1 * b + inv_normal2 * c,
    };
    
    let step_x = build_bundle(step_x_w0, step_x_w1, step_x_w2);
    let step_y = build_bundle(step_y_w0, step_y_w1, step_y_w2);
    let mut row_attrs = build_bundle(w0_row, w1_row, w2_row);
    
    if !camera_info.render_config.affine_color {
      row_attrs.color = (v0.color.to_vec4() + v1.color.to_vec4() + v2.color.to_vec4()) / 3.0;
    }
    
    // the incredible loop
    for sy in min.1..=max.1 {
    
      let mut attrs = row_attrs;
      let mut row_idx = sy * screen_size.0 + min.0;
      
      for _ in min.0..=max.0 {
        let is_inside_tri = (attrs.weight0 >= 0.0) && (attrs.weight1 >= 0.0) && (attrs.weight2 >= 0.0);
        
        if is_inside_tri && attrs.z < self.depth_buffer[row_idx] {
          self.depth_buffer[row_idx] = attrs.z;
          
          buffer[row_idx] = self.pixel_shader(camera_info, &attrs, texture_id, texture_size);
          // buffer[row_idx] = attrs.color.to_u32();
        }
        
        attrs += step_x;
        row_idx += 1;
      }
      
      row_attrs += step_y;
    }
    
  }
  
  fn pixel_shader(&mut self, _camera: &CameraInfo, attrs: &AttributeBundle,
                  texture_id: usize, texture_size: (usize, usize)) -> u32 {
    let uv_normalized = attrs.uv / attrs.z;
    let u_index = uv_normalized.x as usize % texture_size.0;
    let v_index = uv_normalized.y as usize % texture_size.1;
    
    let tex = self.tm.at_raw(texture_id, u_index, v_index);
    let darkness = 255 - ((attrs.normal.normalize().y + 1.0)/2.0 * 255.0) as i32; // [0, 255]
    
    let r = (((attrs.color.x as i32 * tex.r as i32) >> 8) - darkness).max(0) as u8;
    let g = (((attrs.color.y as i32 * tex.g as i32) >> 8) - darkness).max(0) as u8;
    let b = (((attrs.color.z as i32 * tex.b as i32) >> 8) - darkness).max(0) as u8;
    
    b as u32 | (g as u32) << 8 | (r as u32) << 16
  }
}


