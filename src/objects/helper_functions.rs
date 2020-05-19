use super::super::vertex::Vertex;

// Generate texture coordinates from vertex positions
// The resulting tex coords will be the vertex position mapped from the range [min,max] to [0,1]
// A range of 0 for positions will not cause a crash but the resulting tex coords will be garbage
pub fn gen_tex_coords(vertices: &mut[Vertex]) {
    if vertices.len() >= 1 {
        let mut min_x = vertices[0].position.x;
        let mut max_x = vertices[0].position.x;
        let mut min_y = vertices[0].position.y;
        let mut max_y = vertices[0].position.y;
        for vertex in &*vertices {
            if vertex.position.x < min_x {
                min_x = vertex.position.x;
            } else if vertex.position.x > max_x {
                max_x = vertex.position.x;
            }
            if vertex.position.y < min_y {
                min_y = vertex.position.y;
            } else if vertex.position.y > max_y {
                max_y = vertex.position.y;
            }
        }
        let mut diff_x = max_x-min_x;
        let mut diff_y = max_y-min_y;

        if diff_x==0.0 {diff_x=1.0;}
        if diff_y==0.0 {diff_y=1.0;}

        for vertex in vertices {
            vertex.tex_coords_or_color[0] = (vertex.position.x-min_x)/diff_x;
            vertex.tex_coords_or_color[1] = 1.0-(vertex.position.y-min_y)/diff_y;
        }
    }
}