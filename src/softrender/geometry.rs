use crate::softrender::{Vec2, Vec3, Vertex, Color};

#[derive(Debug, Clone, Copy)]
pub struct ModelInfo {
  /// This model's id. Used for UnifiedGeometryBuffer's internal array ordering.
  id: usize,
  /// Where this model starts in the global vertex buffer.
  pub base_vertex: usize,
  /// Where this model's vertices' indices start.
  pub index_start: usize,
  /// How many indices to draw.
  pub index_count: usize,
  /// Texture id used by this model.
  pub texture_id: usize,
  
  pub min_extents: Vec3,
  pub max_extents: Vec3,
}

#[derive(Default)]
pub struct UnifiedGeometryBuffer {
  /// Every vertex for every model, one after the other.
  pub vertices: Vec<Vertex>,
  /// Contains triplets of vertex indices for every triangle in a model, stored sequentially through the models.
  pub indices: Vec<usize>,
  /// Metadata to find specific models.
  pub models: Vec<ModelInfo>,
  models_loaded: usize,
}

impl UnifiedGeometryBuffer {
  pub fn load_model_obj(&mut self, file_path: impl AsRef<std::path::Path>) -> std::io::Result<usize> {
    self.load_textured_model_obj(file_path, 0)
  }
  
  pub fn load_textured_model_obj(&mut self, file_path: impl AsRef<std::path::Path>, texture_id: usize) -> std::io::Result<usize> {
    use std::fs::File;
    use std::io::prelude::*;
    
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    
    file.read_to_string(&mut contents)?;
    
    let mut m_info = ModelInfo{
      id: self.models_loaded,
      base_vertex: self.vertices.len(),
      index_start: self.indices.len(),
      index_count: 0,
      texture_id,
      min_extents: Vec3{ x:  f32::INFINITY, y:  f32::INFINITY, z:  f32::INFINITY },
      max_extents: Vec3{ x: -f32::INFINITY, y: -f32::INFINITY, z: -f32::INFINITY },
    };
    self.models_loaded += 1;
    
    let mut indices = Vec::<usize>::new();
    let mut vertices = Vec::<Vertex>::new();
    let mut positions = Vec::<Vec3>::new();
    let mut normals = Vec::<Vec3>::new();
    let mut uvs = Vec::<Vec2>::new();
    
    for line in contents.split('\n') {
      if line.len() == 0 { continue }
      let mut words = line.split_whitespace();
      // println!("{:?}", words);
      match words.next().unwrap_or("") {
        "v" => { // vertex
          let x_string = words.next().unwrap();
          let y_string = words.next().unwrap();
          let z_string = words.next().unwrap();
          let x: f32 = x_string.parse().unwrap();
          let y: f32 = y_string.parse().unwrap();
          let z: f32 = z_string.parse().unwrap();
          
          m_info.min_extents.x = m_info.min_extents.x.min(x);
          m_info.min_extents.y = m_info.min_extents.y.min(y);
          m_info.min_extents.z = m_info.min_extents.z.min(z);
          m_info.max_extents.x = m_info.max_extents.x.max(x);
          m_info.max_extents.y = m_info.max_extents.y.max(y);
          m_info.max_extents.z = m_info.max_extents.z.max(z);
          
          positions.push(Vec3{x, y, z});
        },
        "vt" => {
          let u_string = words.next().unwrap();
          let v_string = words.next().unwrap();
          let u: f32 = u_string.parse().unwrap();
          let v: f32 = v_string.parse().unwrap();
          
          uvs.push(Vec2{x: u, y: v});
        },
        "vn" => {
          let x_string = words.next().unwrap();
          let y_string = words.next().unwrap();
          let z_string = words.next().unwrap();
          let x: f32 = x_string.parse().unwrap();
          let y: f32 = y_string.parse().unwrap();
          let z: f32 = z_string.parse().unwrap();
          
          let mut normal = Vec3{x, y, z};
          normal = normal / normal.magnitude();
          
          normals.push(normal);
        },
        "f" => {
          let verts: Vec<&str> = words.collect();
          match verts.len() {
            ..3 => (),
            3 => {
              m_info.index_count += self.parse_triangle_obj(verts.as_slice(), &mut indices, &mut vertices, &positions, &normals, &uvs);
            },
            4 => {
              m_info.index_count += self.parse_quad_obj(verts.as_slice(), &mut indices, &mut vertices, &positions, &normals, &uvs);
            },
            5.. => (),
          }
        },
        _ => (),
      }
    }
    
    self.vertices.append(&mut vertices);
    self.indices.append(&mut indices);
    
    println!("{:?}", m_info);
    
    self.models.push(m_info);
    
    Ok(m_info.id)
  }
  
  pub fn get_model_index(&mut self, id: usize) -> Option<usize> {
    let mut left = 0;
    let mut right = self.models.len() - 1;
    let mut middle;
    
    while left <= right {
      middle = (right + left) / 2;
      
      if self.models[middle].id == id {
        return Some(middle);
      } else if self.models[middle].id < id {
        left = middle + 1;
      } else { // self.models[middle].id > id
        right = middle - 1;
      }
    }
    
    return None;
  }

  pub fn get_model(&mut self, id: usize) -> Option<ModelInfo> {
    if let Some(model_index) = self.get_model_index(id) {
      return Some(self.models[model_index]);
    }
    return None;
  }
  
  pub fn remove_model(&mut self, id: usize) -> bool {
    let Some(model_index) = self.get_model_index(id) else {
      return false;
    };
    
    let model_info = self.models[model_index];
    
    // removing triangle vertex indices
    let indices_range = (model_info.index_start)..(model_info.index_start + model_info.index_count);
    self.indices.drain(indices_range);
    
    // removing vertices
    let vertex_count = if model_index == self.models.len() - 1 {
      self.vertices.len() - model_info.base_vertex
    } else {
      self.models[model_index + 1].base_vertex - model_info.base_vertex
    };
    let vertices_range = (model_info.base_vertex)..(model_info.base_vertex + vertex_count);
    self.vertices.drain(vertices_range);
    
    // fixing ids for affected guys
    for i in (model_index + 1)..self.models.len() {
      self.models[i].index_start -= model_info.index_count;
      self.models[i].base_vertex -= vertex_count;
    }
    
    // removing model info
    self.models.remove(model_index);
    
    return true;
  }
  
  fn parse_triangle_obj(&mut self, verts: &[&str], indices: &mut Vec<usize>, vertices: &mut Vec<Vertex>,
                        positions: &Vec<Vec3>, normals: &Vec<Vec3>, uvs: &Vec<Vec2>) -> usize {
    for index in verts {
      let indexes_string: Vec<&str> = index.split('/').collect();
      let indexes: Vec<usize> = indexes_string.into_iter().map(|s: &str| s.parse::<usize>().unwrap() - 1).collect();
      
      let pos = positions[indexes[0]];
      
      indices.push(vertices.len());
      vertices.push( Vertex{
        pos,
        uv: uvs[indexes[1]],
        normal: normals[indexes[2]],
        color: Color{
          r: 255, g: 255, b: 255,
          // r: ((pos.z * 7120.0).abs() % 255.0) as u8,
          // g: ((pos.z * 13233.3).abs() % 255.0) as u8,
          // b: ((pos.z * 155999.5).abs() % 255.0) as u8,
          a: 255,
        },
      });
    }
        
    return 3;
  }

  fn parse_quad_obj(&mut self, verts: &[&str], indices: &mut Vec<usize>, vertices: &mut Vec<Vertex>,
                    positions: &Vec<Vec3>, normals: &Vec<Vec3>, uvs: &Vec<Vec2>) -> usize {
    let tri2 = [verts[0], verts[2], verts[3]];
    
    self.parse_triangle_obj(&verts[0..3], indices, vertices, positions, normals, uvs);
    self.parse_triangle_obj(&tri2, indices, vertices, positions, normals, uvs);
    
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
  
  (p.x - a.x) * (b.y - a.y) - (p.y - a.y) * (b.x - a.x)
}
#[inline(always)]
pub fn edge_function_raw(a: &Vec2, b: &Vec2, px: f32, py: f32) -> f32 {
  (px - a.x) * (b.y - a.y) - (py - a.y) * (b.x - a.x)
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
