use crate::softrender::{Vec2, Vec3, Vertex};

#[derive(Debug, Clone, Copy)]
pub struct ModelInfo {
  pub base_vertex: usize, // Where this model starts in the global vertex buffer
  pub index_start: usize, // Where this model's indices start
  pub index_count: usize, // How many indices to draw
  pub min_extents: Vec3,
  pub max_extents: Vec3,
}

#[derive(Default)]
pub struct UnifiedGeometryBuffer {
  pub vertices: Vec<Vertex>, // Every vertex for every model, one after the other
  pub indices: Vec<usize>,   // Every index for every model, one after the other
  pub models: Vec<ModelInfo>, // Metadata to find specific models
}

impl UnifiedGeometryBuffer {
  pub fn load_obj(&mut self, file_path: String) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::prelude::*;
    
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    
    file.read_to_string(&mut contents)?;
    
    let mut m_info = ModelInfo{
      base_vertex: self.vertices.len(),
      index_start: self.indices.len(),
      index_count: 0,
      min_extents: Vec3{ x:  f32::INFINITY, y:  f32::INFINITY, z:  f32::INFINITY },
      max_extents: Vec3{ x: -f32::INFINITY, y: -f32::INFINITY, z: -f32::INFINITY },
    };
    
    for line in contents.split('\n') {
      if line.len() == 0 { continue }
      let mut words = line.split_whitespace();
      // println!("{:?}", words);
      match words.next().unwrap_or("") {
        "v" => { // vertex
          let xs = words.next().unwrap();
          let ys = words.next().unwrap();
          let zs = words.next().unwrap();
          {
            let x: f32 = xs.parse().unwrap();
            let y: f32 = ys.parse().unwrap();
            let z: f32 = zs.parse().unwrap();
            
            m_info.min_extents.x = m_info.min_extents.x.min(x);
            m_info.min_extents.y = m_info.min_extents.y.min(y);
            m_info.min_extents.z = m_info.min_extents.z.min(z);
            m_info.max_extents.x = m_info.max_extents.x.max(x);
            m_info.max_extents.y = m_info.max_extents.y.max(y);
            m_info.max_extents.z = m_info.max_extents.z.max(z);
            
            self.vertices.push( Vertex{
              pos: Vec3{x, y, z},
              color: Vec3{x: (z * 1.2) % 1.0, y: (z * 1.3) % 1.0, z: (z * 1.5) % 1.0}
            });
          }
        },
        "f" => {
          let verts: Vec<&str> = words.collect();
          match verts.len() {
            ..3 => (),
            3 => {
              m_info.index_count += self.parse_triangle_obj(verts.as_slice());
            },
            4 => {
              m_info.index_count += self.parse_quad_obj(verts.as_slice());
            },
            5.. => (),
          }
          // m_info.index_count += 3;
        },
        _ => (),
      }
      // println!("{}", line);
    }
    
    println!{"{:?}", m_info};
    
    self.models.push(m_info);
    
    Ok(())
  }
  
  // pub fn init(&mut self) {
  //   let _ = self.load_obj("src/monke.obj".to_string());
  // }
  
  // pub fn load_obj(&mut self, file_path: String) {
  //   let _ = self.load_obj(file_path);
  // }
  
  fn parse_triangle_obj(&mut self, verts: &[&str]) -> usize {
    for index in verts {
      let index_string = index.split('/').next().unwrap();
      self.indices.push(index_string.parse::<usize>().unwrap() - 1);
    }
        
    return 3;
  }

  fn parse_quad_obj(&mut self, verts: &[&str]) -> usize {
    let tri2 = [verts[0], verts[2], verts[3]];
    
    self.parse_triangle_obj(&verts[0..3]);
    self.parse_triangle_obj(&tri2);
    
    return 6;
  }
}

#[inline(always)]
pub fn edge_function(a: &Vec2, b: &Vec2, p: &Vec2) -> f32 {
  // expanded math of (b - a).orthogonal().dot(p - a)
  // old code:
  //   let bottom_side = (v0 - v2).orthogonal();
  //   let right_side = v1 - v2;
  //   return bottom_side.dot(right_side);
  
  ((p.x - a.x) * (b.y - a.y) - (p.y - a.y) * (b.x - a.x)) / 2.0
}

#[inline(always)]
pub fn edge_function_raw(a: &Vec2, b: &Vec2, px: f32, py: f32) -> f32 {
  ((px - a.x) * (b.y - a.y) - (py - a.y) * (b.x - a.x)) / 2.0
}

#[inline(always)]
pub fn translate_to_screen(vert: &Vec3, screen_size: &(usize, usize)) -> Vec2 {
  let fov = 90.0f32.to_radians();
  
  let screenheight_world = f32::tan(fov / 2.0) * 2.0;
  let pixels_per_world_unit = screen_size.1 as f32 / screenheight_world / -vert.z;
  
  let mut pixel_offset = Vec2{ x: vert.x, y: vert.y } * pixels_per_world_unit;
  pixel_offset.y *= -1.0;
  let final_point = Vec2{ x: screen_size.0 as f32 / 2.0 + pixel_offset.x, y: screen_size.1 as f32 / 2.0 + pixel_offset.y};
      
  final_point
}
