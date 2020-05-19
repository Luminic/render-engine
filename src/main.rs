use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
};

use cgmath::*;

mod texture;
// use texture::Texture;

mod vertex;

mod point;
use point::Point;

mod camera;
use camera::*;

mod uniforms;
// use uniforms::Uniforms;

mod window;
// use state::State;

mod renderer;
use renderer::*;

mod objects;
use objects::polygons::*;
use objects::lines::*;
use objects::bezier_curves::*;

fn main() {
    let event_loop = EventLoop::new();
    let format = wgpu::TextureFormat::Bgra8UnormSrgb;
    
    use futures::executor::block_on;
    
    let mut window = window::Window::new(&event_loop, format);

    let mut renderer = block_on(Renderer::new(10000, 15000, format, Some(window.get_surface())));

    window.init_swapchain(renderer.get_device());

    let mut camera = Camera::new();

    let mut camera_controller = CameraController::new(0.04, 1.04, 0.06);

    renderer.load_texture(include_bytes!("textures/awesomeface_with_transparency.png"), String::from("awesomeface.png"));
    renderer.load_texture(include_bytes!("textures/happy-tree.png"), String::from("happy-tree.png"));

    let t0 = Triangle::new(Point{x:-0.3, y:-0.3}, Point{x: 0.3, y:-0.3}, Point{x: 0.0, y: 0.3}, None, Some(&[0.1,0.0,0.8,0.5]));
    let r1 = Rectangle::new(Point{x:-0.5, y: 0.5}, Point{x: 0.5, y:-0.5}, Some(String::from("happy-tree.png")), None);
    let r2 = Rectangle::new(Point{x:-0.5, y: 0.5}, Point{x: 0.5, y:-0.5}, Some(String::from("awesomeface.png")), None);
    let poly = Polygon::new(
        &[
            Point{x: 0.00, y: 1.00},
            Point{x:-0.24, y: 0.31},
            Point{x:-1.00, y: 0.31},
            Point{x:-0.38, y:-0.10},
            Point{x:-0.62, y:-0.79},
            Point{x: 0.00, y:-0.36},

            Point{x: 0.62, y:-0.79},
            Point{x: 0.38, y:-0.10},
            Point{x: 1.00, y: 0.31},
            Point{x: 0.24, y: 0.31},
        ],
        Some(String::from("happy-tree.png")),
        None,
    );
    let l0 = Line::new(Point{x:-0.3, y:0.3}, Point{x:0.3, y:-0.3}, 0.01, Some(String::from("happy-tree.png")), None);

    let bezier_points = [Point{x:-0.3, y:-0.3}, Point{x: 0.0, y: 0.3}, Point{x: 0.5, y:-0.3}, Point{x:0.15, y:-0.5}, Point{x:-0.2, y:-0.3}, Point{x: 0.2, y:0.2}, Point{x: 0.4, y:0.1}];
    let l1 = LineStrip::new(
        &bezier_points,
        0.01, None, Some(&[0.0,0.0,0.0,1.0])
    );
    let b0 = BezierCurve::new(
        &bezier_points,
        1000, 0.01, None, Some(&[0.0,1.0,1.0,1.0])
    );

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent { ref event, window_id } if window_id == window.winit_window.id() => {
                if !camera_controller.process_events(&event) {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::KeyboardInput { input, .. } => {
                            match input {
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                } => *control_flow = ControlFlow::Exit,
                                _ => {},
                            }
                        },
                        WindowEvent::Resized(physical_size) => {
                            window.resize(*physical_size, renderer.get_device());
                            camera.aspect_ratio = window.get_aspect_ratio();
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            window.resize(**new_inner_size, renderer.get_device());
                            camera.aspect_ratio = window.get_aspect_ratio();
                        }
                        _ => {},
                    }
                }
            }
            Event::RedrawRequested(_) => {
                let frame = DrawableFrame::from_sc_output(window.get_next_frame().unwrap());
                camera_controller.update_camera(&mut camera);
                renderer.update(&camera);
                
                renderer.begin_render(frame);
                renderer.draw(&poly,
                    Some(&UsableTransform{
                        translation: vec2(-0.5,-0.5),
                        scale: vec2(0.5,0.5),
                        rotation: 0.0,
                    })
                );
                renderer.draw(&r1, 
                    Some(&UsableTransform{
                        translation: vec2(0.5,0.0),
                        scale: vec2(1.0,0.5),
                        rotation: 0.7814,
                    })
                );
                renderer.draw(&r2, 
                    Some(&UsableTransform{
                        translation: vec2(1.0,-0.5),
                        scale: vec2(1.0,1.0),
                        rotation: 0.0,
                    })
                );
                renderer.draw(&t0, None);
                renderer.draw(&l0, None);
                let bezier_transform = UsableTransform{
                    translation: vec2(-0.6,0.6),
                    scale: vec2(1.0,1.0),
                    rotation: 0.0,
                };
                renderer.draw(&l1, Some(&bezier_transform));
                renderer.draw(b0.get_line_strip(), Some(&bezier_transform));
                renderer.end_render();
            }
            Event::MainEventsCleared => {
                window.winit_window.request_redraw();
            }
            _ => {},
        }
    });
}
