use cgmath::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Uniforms {
    camera_transform: Matrix4::<f32>,
}

unsafe impl bytemuck::Pod for Uniforms {}
unsafe impl bytemuck::Zeroable for Uniforms {}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            camera_transform: Matrix4::identity(),
        }
    }
}

use super::camera::Camera;
impl Uniforms {
    pub fn set_camera(&mut self, camera: &Camera) {
        self.camera_transform = Matrix4::from(camera.get_view_matrix());
    }
}