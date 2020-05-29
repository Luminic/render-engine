use super::super::vertex::*;
use super::super::point::*;
use super::super::renderer;

use super::helper_functions::gen_tex_coords;

/*
Lines are constructed using two triangle primitives to make rectangles
This is because the primitive LineList doesn't allow users to change line width (stuck a 1 px)

Pretty much a non axis aligned Rectangle

Two adjacent line segments  will be connected by another set of indices making a third rectangle

(The line segments should be intersecting at the *, I pulled it apart to make it clearer)

0-----1
|\    |
| \   |
|  \  |        |
|   \ |        |
|    \|        |
3--*--2        | Line direction
| `-, |        |
0--*--1        |
|\    |        |
| \   |        V
|  \  |
|   \ |
|    \|
3-----2
*/
pub struct Line {
    indices: [u16; 6],
    vertices: [Vertex; 4],
    texture:Option<String>,
}

impl Line {
    pub fn new(p0: Point, p1: Point, width: f32, texture: Option<String>, color: Option<&[f32;4]>) -> Self {
        let indices: [u16; 6] = [
            0, 1, 2,
            2, 3, 0,
        ];

        let mut norm = Point {
            x: p0.y-p1.y,
            y: p1.x-p0.x,
        };
        norm.normalize();
        norm *= width/2.0;

        let mut vertices = [
            Vertex { position: p0, texture_binding:-1, tex_coords_or_color: [1.0, 1.0, 0.0, 1.0] },
            Vertex { position: p0, texture_binding:-1, tex_coords_or_color: [1.0, 1.0, 0.0, 1.0] },
            Vertex { position: p1, texture_binding:-1, tex_coords_or_color: [1.0, 1.0, 0.0, 1.0] },
            Vertex { position: p1, texture_binding:-1, tex_coords_or_color: [1.0, 1.0, 0.0, 1.0] },
        ];

        vertices[0].position -= norm;
        vertices[1].position += norm;
        vertices[2].position += norm;
        vertices[3].position -= norm;
        
        match color {
            Some(col) => {
                for vert in &mut vertices {
                    vert.texture_binding = -1;
                    vert.tex_coords_or_color = *col;
                }
            },
            None => gen_tex_coords(&mut vertices),
        };
        Self {
            indices,
            vertices,
            texture,
        }
    }
}

impl<'a> renderer::Drawable<'a> for Line {
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

pub struct LineStrip {
    indices: Vec<u16>,
    vertices: Vec<Vertex>,
    texture: Option<String>,
}

impl LineStrip {
    pub fn add_point(&mut self, point: Point, prev_point: Point, width: f32, color: Option<&[f32;4]>) {
        let mut norm = Point {
            x: point.y-prev_point.y,
            y: prev_point.x-point.x,
        };
        norm.normalize();
        norm *= width/2.0;
        let vertices_starting_index = self.vertices.len();
        self.vertices.push(Vertex{position: prev_point-norm, texture_binding: 0, tex_coords_or_color: [0.0,0.0,0.0,0.0]});
        self.vertices.push(Vertex{position: prev_point+norm, texture_binding: 0, tex_coords_or_color: [0.0,0.0,0.0,0.0]});
        self.vertices.push(Vertex{position:      point+norm, texture_binding: 0, tex_coords_or_color: [0.0,0.0,0.0,0.0]});
        self.vertices.push(Vertex{position:      point-norm, texture_binding: 0, tex_coords_or_color: [0.0,0.0,0.0,0.0]});

        match color {
            Some(col) => {
                for vert in &mut self.vertices[vertices_starting_index..] {
                    vert.texture_binding = -1;
                    vert.tex_coords_or_color = *col;
                }
            },
            None => {
                // If an image is used, tex coords will have to be recalculated
                // Because add_points mught be used multiple times in a row, its best to recalculate tex coords after all the points have been added
            },
        }
        
        // Add the new quad to indices
        self.indices.push(vertices_starting_index as u16 +0);
        self.indices.push(vertices_starting_index as u16 +1);
        self.indices.push(vertices_starting_index as u16 +2);
        self.indices.push(vertices_starting_index as u16 +2);
        self.indices.push(vertices_starting_index as u16 +3);
        self.indices.push(vertices_starting_index as u16 +0);

        // If there is already a line segment connect the two together with another quad
        if vertices_starting_index >= 4 {
            self.indices.push(vertices_starting_index as u16 -1);
            self.indices.push(vertices_starting_index as u16 -2);
            self.indices.push(vertices_starting_index as u16 +1);
            self.indices.push(vertices_starting_index as u16 +1);
            self.indices.push(vertices_starting_index as u16 +0);
            self.indices.push(vertices_starting_index as u16 -1);
        }
    }

    pub fn new(points: &[Point], width: f32, texture: Option<String>, color: Option<&[f32;4]>) -> Self {
        assert!(points.len() >= 2);
        let indices = vec![];
        let vertices = vec![];
        let mut result = Self {
            indices,
            vertices,
            texture,
        };
        for i in 1..points.len() {
            result.add_point(points[i], points[i-1], width, color);
        }
        if color.is_none() {
            gen_tex_coords(&mut result.vertices); // Calculate texture coordinates
        }
        result
    }
}

impl<'a> renderer::Drawable<'a> for LineStrip {
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