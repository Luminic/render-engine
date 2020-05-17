use winit::{
    event::*,
};

use cgmath::*;

use super::vertex::*;

macro_rules! create_transformation_matrix {
    ($translation: expr, $scale: expr, $rotation: expr) => {{
        let translation = Matrix3::<f32>::new(
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            $translation.x, $translation.y, 1.0,
        );
        let rotation = Matrix3::from(Matrix2::from_angle(Rad($rotation)));
        let scale = Matrix3::<f32>::new(
            $scale.x, 0.0,      0.0,
            0.0,      $scale.y, 0.0,
            0.0,      0.0,      1.0,
        );
        translation*rotation*scale
    }}
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UsableTransform {
    pub translation: Vector2<f32>,
    pub scale: Vector2<f32>,
    pub rotation: f32,
}

impl UsableTransform {
    pub fn transform_point_with_matrix(point: &mut Point, matrix: &Matrix3<f32>) {
        let new_position = matrix * vec3(point.x, point.y, 1.0);
        point.x = new_position.x;
        point.y = new_position.y;
    }

    pub fn get_transformation_matrix(&self) -> Matrix3<f32> {
        create_transformation_matrix!(self.translation, self.scale, self.rotation)
    }
}

#[allow(dead_code)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum ASPECT_RATIO_BEHAVIOR {
    FixedWidth,
    FixedHeight,
    NoEffect,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Camera {
    center: Vector2<f32>,
    // scr_width / scr_height
    pub aspect_ratio: f32,
    pub aspect_ratio_behavior: ASPECT_RATIO_BEHAVIOR,
    scaling: f32,
    rotation: f32,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            center: Vector2::<f32>::new(0.0,0.0),
            aspect_ratio: 1.0,
            aspect_ratio_behavior: ASPECT_RATIO_BEHAVIOR::FixedWidth,
            scaling: 1.0,
            rotation: 0.0,
        }
    }
    pub fn get_view_matrix(&self) -> Matrix3<f32> {
        let scaling = match self.aspect_ratio_behavior {
            ASPECT_RATIO_BEHAVIOR::FixedWidth => {
                vec2(self.scaling, self.scaling*self.aspect_ratio)
            }
            ASPECT_RATIO_BEHAVIOR::FixedHeight => {
                vec2(self.scaling/self.aspect_ratio, self.scaling)
            }
            ASPECT_RATIO_BEHAVIOR::NoEffect => {
                vec2(self.scaling, self.scaling)
            }
        };
        // The creation of this transformation is slightly different to that of create_transformation_matrix!
        // In this case, the matrix muliplication is translation, rotation, then scale so the world rotates and scales arond the center of the screenm not (0,0)
        // In create_transformation_matrix! its the other way around
        let scale = Matrix3::<f32>::new(
            scaling.x, 0.0,       0.0,
            0.0,       scaling.y, 0.0,
            0.0,       0.0,       1.0,
        );
        let rotation = Matrix3::from(Matrix2::from_angle(Rad(self.rotation)));
        let translation = Matrix3::<f32>::new(
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            self.center.x, self.center.y, 1.0,
        );
        scale*rotation*translation
    }
}

pub struct CameraController {
    pub move_speed: f32,
    pub zoom_speed: f32,
    pub rotation_speed: f32,

    move_up: bool,
    move_down: bool,
    move_left: bool,
    move_right: bool,

    zoom_in: bool,
    zoom_out: bool,

    rotate_cw: bool,
    rotate_ccw: bool,
}

impl CameraController {
    pub fn new(move_speed: f32, zoom_speed: f32, rotation_speed: f32) -> Self {
        Self {
            move_speed,
            zoom_speed,
            rotation_speed,

            move_up: false,
            move_down: false,
            move_left: false,
            move_right: false,

            zoom_in: false,
            zoom_out: false,
            
            rotate_cw: false,
            rotate_ccw: false,
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state,
                    virtual_keycode: Some(keycode),
                    ..
                },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                        self.move_up = is_pressed;
                        true
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.move_left = is_pressed;
                        true
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.move_down = is_pressed;
                        true
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.move_right = is_pressed;
                        true
                    }
                    VirtualKeyCode::Space => {
                        self.zoom_in = is_pressed;
                        true
                    }
                    VirtualKeyCode::LShift => {
                        self.zoom_out = is_pressed;
                        true
                    }
                    VirtualKeyCode::Q => {
                        self.rotate_cw = is_pressed;
                        true
                    }
                    VirtualKeyCode::E => {
                        self.rotate_ccw = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub fn update_camera(&self, camera: &mut Camera) {
        // Scale the move speed so an edge will move the same # of pixels whether the camera is zoomed in or super zoomed out
        let move_speed = self.move_speed / camera.scaling;
        // I want the rotation and scaling to be over the center of the screen, but for moving to also always be horizontal or vertical relative to the window
        // So the rotation of the camera has to be taken into account and negated when moving the camera
        let sin_cos_of_angle = camera.rotation.sin_cos();
        let vert_dir = vec2(sin_cos_of_angle.0,sin_cos_of_angle.1);
        let horz_dir = vec2(-sin_cos_of_angle.1,sin_cos_of_angle.0);
        
        if self.move_up {
            camera.center -= move_speed*vert_dir;
        }
        if self.move_left {
            camera.center -= move_speed*horz_dir;
        }
        if self.move_down {
            camera.center += move_speed*vert_dir;
        }
        if self.move_right {
            camera.center += move_speed*horz_dir;
        }
        if self.zoom_in {
            camera.scaling /= self.zoom_speed;
        }
        if self.zoom_out {
            camera.scaling *= self.zoom_speed;
        }
        if self.rotate_cw {
            camera.rotation += self.rotation_speed;
        }
        if self.rotate_ccw {
            camera.rotation -= self.rotation_speed;
        }
    }
}
