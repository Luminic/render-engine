use super::super::vertex::*;
use super::super::point::*;
use super::super::renderer;

use super::lines::LineStrip;


// pub type BezierEndPoint = Point;
// pub type BezierControlPoint = Point;

/**
 * Calculate an nth degree Bernstien polynomial at u
 * result must be an array from 0..(n+1)
 */
fn get_bernstien_polynomial_results(n: usize, u: f32, result: &mut [f32]) {
    result[0] = 1.0;
    let u1 = 1.0-u;
    for j in 1..(n+1) {
        let mut saved = 0.0;
        for k in 0..j {
            let temp = result[k];
            result[k] = saved+u1*temp;
            saved = u*temp;
        }
        result[j] = saved;
    }
}

/**
 * C(u) = Sum from i=0 to n (B_(i,n)(u)*P_i)
 * where B_(i,n)(u) = (n choose i)*u^i*(1-u)^(n-i)
 * and P_i are the 
 */
pub struct BezierCurve {
    control_points: Vec<Point>,
    quality: usize,
    resulting_points: Vec<Point>,

    line_strip: LineStrip,
}

impl BezierCurve {
    // Contol points are the number of control points on the curve
    // Quality is the number of vertices generated from the curve excluding endpoints
    pub fn new(control_points: &[Point], quality: usize, width: f32, texture: Option<String>, color: Option<&[f32;4]>) -> Self {
        let mut resulting_points = vec![control_points[0]];
        let step = 1.0/(quality as f32 +1.0);
        let mut u = step;
        let mut bernstien_polynomial_results = vec![0.0; control_points.len()];
        for _ in 0..quality {
            get_bernstien_polynomial_results(control_points.len()-1, u, &mut bernstien_polynomial_results);
            u += step;
            let mut p = Point{x:0.0, y:0.0};
            for i in 0..control_points.len() {
                p += control_points[i]*bernstien_polynomial_results[i];
            }
            resulting_points.push(p);
        }
        resulting_points.push(control_points[control_points.len()-1]);
        let line_strip = LineStrip::new(&resulting_points, width, texture, color);

        Self {
            control_points: control_points.to_vec(),
            quality,
            resulting_points,

            line_strip,
        }
    }

    pub fn get_line_strip(&self) -> &LineStrip {
        &self.line_strip
    }
}