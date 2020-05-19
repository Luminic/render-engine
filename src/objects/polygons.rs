use super::super::vertex::*;
use super::super::point::*;
use super::super::renderer;

use super::helper_functions::gen_tex_coords;

pub struct Rectangle {
    indices: [u16; 6],
    vertices: [Vertex; 4],
    texture:Option<String>,
}

impl Rectangle {
    pub fn new(top_left:Point, bottom_right:Point, texture:Option<String>, color:Option<&[f32;4]>) -> Self {
        let indices: [u16; 6] = [
            0, 1, 3,
            1, 3, 2,
        ];

        let vertices:[Vertex; 4];

        match color {
            Some(col) => {
                vertices = [
                    Vertex { position: Point{x:top_left.x,     y:top_left.y},     texture_binding:-1, tex_coords_or_color: *col },
                    Vertex { position: Point{x:bottom_right.x, y:top_left.y},     texture_binding:-1, tex_coords_or_color: *col },
                    Vertex { position: Point{x:bottom_right.x, y:bottom_right.y}, texture_binding:-1, tex_coords_or_color: *col },
                    Vertex { position: Point{x:top_left.x,     y:bottom_right.y}, texture_binding:-1, tex_coords_or_color: *col },
                ];
                if texture.is_some() {
                    // Placeholder; replace wth a proper error message later
                    let texture:Option<String> = None;
                    return Self {
                        indices,
                        vertices,
                        texture,
                    };
                }
            }
            None => {
                vertices = [
                    Vertex { position: Point{x:top_left.x,     y:top_left.y},     texture_binding:0, tex_coords_or_color: [0.0, 0.0, 0.0, 1.0] },
                    Vertex { position: Point{x:bottom_right.x, y:top_left.y},     texture_binding:0, tex_coords_or_color: [1.0, 0.0, 0.0, 1.0] },
                    Vertex { position: Point{x:bottom_right.x, y:bottom_right.y}, texture_binding:0, tex_coords_or_color: [1.0, 1.0, 0.0, 1.0] },
                    Vertex { position: Point{x:top_left.x,     y:bottom_right.y}, texture_binding:0, tex_coords_or_color: [0.0, 1.0, 0.0, 1.0] },
                ];
            }
        }
        
        Self {
            indices,
            vertices,
            texture
        }
    }
}

impl<'a> renderer::Drawable<'a> for Rectangle {
    fn get_vertex_information(&'a self) -> (&'a[u16], &'a[Vertex]) {
        (&self.indices, &self.vertices)
    }

    fn get_texture_name(&self) -> Option<String> {
        match &self.texture {
            Some(x) => Some(x.to_string()),
            None => None,
        }
    }
}

pub struct Triangle {
    indices: [u16; 3],
    vertices: [Vertex; 3],
    texture:Option<String>,
}

impl Triangle {
    pub fn new(p0:Point, p1:Point, p2:Point, texture:Option<String>, color:Option<&[f32;4]>) -> Self {
        let indices: [u16; 3] = [0, 1, 2];

        let mut vertices:[Vertex; 3];

        match color {
            Some(col) => {
                vertices = [
                    Vertex { position: p0, texture_binding:-1, tex_coords_or_color: *col },
                    Vertex { position: p1, texture_binding:-1, tex_coords_or_color: *col },
                    Vertex { position: p2, texture_binding:-1, tex_coords_or_color: *col },
                ];
                if texture.is_some() {
                    // Placeholder; replace wth a proper error message later
                    let texture:Option<String> = None;
                    return Self {
                        indices,
                        vertices,
                        texture,
                    };
                }
            }
            None => {
                vertices = [
                    Vertex { position: p0, texture_binding:0, tex_coords_or_color: [0.0, 0.0, 0.0, 1.0] },
                    Vertex { position: p1, texture_binding:0, tex_coords_or_color: [0.0, 0.0, 0.0, 1.0] },
                    Vertex { position: p2, texture_binding:0, tex_coords_or_color: [0.0, 0.0, 0.0, 1.0] },
                ];
                gen_tex_coords(&mut vertices);
            }
        }
        
        Self {
            indices,
            vertices,
            texture
        }
    }
}

impl<'a> renderer::Drawable<'a> for Triangle {
    fn get_vertex_information(&'a self) -> (&'a[u16], &'a[Vertex]) {
        (&self.indices, &self.vertices)
    }

    fn get_texture_name(&self) -> Option<String> {
        match &self.texture {
            Some(x) => Some(x.to_string()),
            None => None,
        }
    }
}

// Calculates whether or not a point is in a triangle using barycentric coordinates
// Inclusive (points on the edge will be counted as in the triangle)
fn point_in_triangle(p:Point, tp0:Point, tp1:Point, tp2:Point) -> bool {
    let double_area = -tp1.y*tp2.x + tp0.y*(-tp1.x + tp2.x) + tp0.x*(tp1.y - tp2.y) + tp1.x*tp2.y;
    let area_sign = double_area.signum();

    let s_p = area_sign*(tp0.y*tp2.x - tp0.x*tp2.y + (tp2.y - tp0.y)*p.x + (tp0.x - tp2.x)*p.y);
    if s_p < 0.0 {return false;}

    let t_p = area_sign*(tp0.x*tp1.y - tp0.y*tp1.x + (tp0.y - tp1.y)*p.x + (tp1.x - tp0.x)*p.y);
    if t_p < 0.0 {return false;}
    
    if s_p + t_p <= double_area.abs() {
        true
    } else {
        false
    }
}

// Calculates whether the angle created by p0-2 is reflex if the points are in ccw order
fn is_reflex_angle(a:Point, b:Point, c:Point) -> bool {
    (b.x - a.x) * (c.y - b.y) - (c.x - b.x) * (b.y - a.y) > 0.0
}

// The polygon cannot have self intersections or self tangencies
// The points must represent the polygon in a ccw order
// If a polygon cannot be made, this function will return false. resulting_indices might have garbage data in it
fn ear_clipping(
    original_points: &[Point], 
    points: &mut Vec<usize>,
    resulting_indices: &mut Vec<u16>) -> bool 
{
    // println!("ec: {}", points.len());
    if points.len() == 4 {
        resulting_indices.push(points[0] as u16);
        resulting_indices.push(points[1] as u16);
        resulting_indices.push(points[3] as u16);
        resulting_indices.push(points[1] as u16);
        resulting_indices.push(points[2] as u16);
        resulting_indices.push(points[3] as u16);
        return true;
    } else if points.len() == 3 {
        resulting_indices.push(points[0] as u16);
        resulting_indices.push(points[1] as u16);
        resulting_indices.push(points[2] as u16);
        return true;
    }
    for i in 0..(points.len() as isize) {
        let mut point_before_index_index = i-1;
        let mut point_after_index_index = i+1;
        if point_before_index_index == -1 {point_before_index_index += points.len() as isize;}
        if point_after_index_index == points.len() as isize {point_after_index_index = 0;}

        let p_bef_idx = points[point_before_index_index as usize];
        let p_cur_idx = points[i as usize];
        let p_aft_idx = points[point_after_index_index as usize];

        let p_bef = original_points[p_bef_idx];
        let p_cur = original_points[p_cur_idx];
        let p_aft = original_points[p_aft_idx];

        // println!("points: {:?}", points);
        // println!("p_idx: {}, {}, {}", p_bef_idx, p_cur_idx, p_aft_idx);

        if is_reflex_angle(p_bef, p_cur, p_aft) {
            let mut is_ear = true;
            // for point_index in 0..original_points.len() {
            for point_index in &*points {
                if *point_index==p_bef_idx || *point_index==p_cur_idx || *point_index==p_aft_idx {
                    continue;
                } else if point_in_triangle(original_points[*point_index], p_bef, p_cur, p_aft) {
                    is_ear = false;
                    break;
                }
            }
            if is_ear {
                points.remove(i as usize);
                resulting_indices.push(p_bef_idx as u16);
                resulting_indices.push(p_cur_idx as u16);
                resulting_indices.push(p_aft_idx as u16);
                // println!("resulting_indices: {:?}", resulting_indices);
                return ear_clipping(original_points, points, resulting_indices);
            }
        }
    }
    false
}

pub struct Polygon {
    indices: Vec<u16>,
    vertices: Vec<Vertex>,
    texture:Option<String>,
}

impl Polygon {
    // The polygon cannot have self intersections or self tangencies
    // The points represent the polygon in a ccw order
    // Failure to follow the above will result in a nonexistent or malformed shape
    pub fn new(points: &[Point], texture:Option<String>, color:Option<&[f32;4]>) -> Self {
        let mut indices: Vec<u16> = vec![];
        let mut point_indexes: Vec<usize> = (0..points.len()).collect();
        // println!("{}", ear_clipping(points, &mut point_indexes, &mut indices));
        ear_clipping(points, &mut point_indexes, &mut indices);
        // println!("{:?}", &indices);

        let mut vertices: Vec<Vertex> = vec![];
        for point in points {
            vertices.push(Vertex { position: *point, texture_binding:-1, tex_coords_or_color: [0.0,0.0,0.0,1.0] });
        }

        match color {
            Some(col) => {
                for vertex in &mut vertices {
                    vertex.tex_coords_or_color = *col;
                }
                if texture.is_some() {
                    // Placeholder; replace wth a proper error message later
                    let texture:Option<String> = None;
                    return Self {
                        indices,
                        vertices,
                        texture,
                    };
                }
            }
            None => {
                gen_tex_coords(&mut vertices);
            }
        }
        
        Self {
            indices,
            vertices,
            texture
        }
    }
}

impl<'a> renderer::Drawable<'a> for Polygon {
    fn get_vertex_information(&'a self) -> (&'a[u16], &'a[Vertex]) {
        (&self.indices, &self.vertices)
    }

    fn get_texture_name(&self) -> Option<String> {
        match &self.texture {
            Some(x) => Some(x.to_string()),
            None => None,
        }
    }
}