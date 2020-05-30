// use super::super::vertex::*;
use super::super::point::*;
use super::super::renderer::*;
use super::super::camera::UsableTransform;

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
    // control_points: Vec<Point>,
    // quality: usize,
    // resulting_points: Vec<Point>,

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
            // control_points: control_points.to_vec(),
            // quality,
            // resulting_points,
            
            line_strip,
        }
    }
    
    pub fn get_line_strip(&self) -> &LineStrip {
        &self.line_strip
    }
    
    pub fn draw(&self, renderer: &mut Renderer, transformation: Option<&UsableTransform>) {
        renderer.draw(&self.line_strip, transformation);
    }
}

pub struct BezierCurveHandle3rdDeg {
    end_point: Point,
    control_point_prev: Point,
    control_point_next: Point,
}

impl BezierCurveHandle3rdDeg {
    pub fn to_bezier_curve(bz_handle0: &BezierCurveHandle3rdDeg, bz_handle1: &BezierCurveHandle3rdDeg, quality: usize, width: f32, texture: Option<String>, color: Option<&[f32;4]>) -> BezierCurve {
        BezierCurve::new(
            &[
                bz_handle0.end_point,
                bz_handle0.control_point_next,
                bz_handle1.control_point_prev,
                bz_handle1.end_point,
            ],
            quality, width, texture, color
        )
    }

    pub fn reverse(&mut self) {
        let tmp = self.control_point_prev;
        self.control_point_prev = self.control_point_next;
        self.control_point_next = tmp;
    }
}

pub struct BezierCurves3rdDeg {
    bezier_handles: Vec<BezierCurveHandle3rdDeg>,
    bezier_curves: Vec<BezierCurve>,
    closed: bool,
}

impl BezierCurves3rdDeg {
    pub fn automatic_bezier_handle(prev: Point, curr: Point, next: Option<Point>) -> BezierCurveHandle3rdDeg {
        let control_point_prev;
        let control_point_next;
        
        let cp_prev_distance = (prev - curr).length()/3.0;
        let mut cp_prev_direction = match next {
            Some(n) => prev - n,
            None => prev - curr,
        };
        cp_prev_direction.normalize();
        
        let cp_next_distance = match next {
            Some(n) => (n-curr).length()/3.0,
            None => cp_prev_distance,
        };
    
        control_point_prev = curr + cp_prev_direction * cp_prev_distance;
        control_point_next = curr - cp_prev_direction * cp_next_distance;

        BezierCurveHandle3rdDeg {
            end_point: curr,
            control_point_prev,
            control_point_next,
        }
    }

    pub fn add_bezier_handle(&mut self, end_point: Point, closed: bool) {
        let bz_handle;
        let bz_handles_len = self.bezier_handles.len();
        if bz_handles_len >= 1 {
            if closed && bz_handles_len >= 2 {
                bz_handle = BezierCurves3rdDeg::automatic_bezier_handle(self.bezier_handles[bz_handles_len-1].end_point, end_point, Some(self.bezier_handles[0].end_point));
            } else {
                bz_handle = BezierCurves3rdDeg::automatic_bezier_handle(self.bezier_handles[bz_handles_len-1].end_point, end_point, None);
            }
            self.bezier_handles.push(bz_handle);
            if bz_handles_len >= 2 {
                self.bezier_handles[bz_handles_len-1] = BezierCurves3rdDeg::automatic_bezier_handle(self.bezier_handles[bz_handles_len-2].end_point, self.bezier_handles[bz_handles_len-1].end_point, Some(self.bezier_handles[bz_handles_len].end_point));
                if closed {
                    self.bezier_handles[0] = BezierCurves3rdDeg::automatic_bezier_handle(self.bezier_handles[bz_handles_len].end_point, self.bezier_handles[0].end_point, Some(self.bezier_handles[1].end_point));
                }
            } else {
                self.bezier_handles[bz_handles_len-1] = BezierCurves3rdDeg::automatic_bezier_handle(self.bezier_handles[bz_handles_len].end_point, self.bezier_handles[bz_handles_len-1].end_point, None);
                self.bezier_handles[bz_handles_len-1].reverse();
            }
        } else {
            bz_handle = BezierCurveHandle3rdDeg {
                end_point,
                control_point_prev: Point{x:0.0,y:0.0},
                control_point_next: Point{x:0.0,y:0.0},
            };
            self.bezier_handles.push(bz_handle);
        }
    }

    pub fn automatic_control_points(end_points: &[Point], closed:bool, quality: usize, width: f32, texture: Option<String>, color: Option<&[f32;4]>) -> Self {
        let mut bz3d = Self{bezier_handles:vec![],bezier_curves:vec![],closed:true};
        for i in 0..end_points.len() {
            bz3d.add_bezier_handle(end_points[i], closed);
        }
        for i in 0..bz3d.bezier_handles.len()-1 {
            bz3d.bezier_curves.push(
                BezierCurveHandle3rdDeg::to_bezier_curve(
                    &bz3d.bezier_handles[i],
                    &bz3d.bezier_handles[i+1],
                    quality, width, texture.clone(), color,
                )
            );
        }
        if closed {
            bz3d.bezier_curves.push(
                BezierCurveHandle3rdDeg::to_bezier_curve(
                    &bz3d.bezier_handles[bz3d.bezier_handles.len()-1],
                    &bz3d.bezier_handles[0],
                    quality, width, texture.clone(), color,
                )
            );
        }
        bz3d
    }
    pub fn draw(&self, renderer: &mut Renderer, transformation: Option<&UsableTransform>) {
        for bz_curve in &self.bezier_curves {
            bz_curve.draw(renderer, transformation);
        }
    }
}