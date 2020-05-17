// mod texture;
use super::texture::Texture;

// mod vertex;
use super::vertex::Vertex;

// mod camera;
use super::camera::*;

// mod uniforms;
use super::uniforms::Uniforms;

use std::collections::{
    HashMap,
    hash_map,
};

const MAX_TEXTURES_PER_DRAW_CALL:usize = 10;
const PLACEHOLDER_TEXTURE_NAME:&str = "placeholder_texture.png";

mod shader {
    use std::fs;
    pub fn create_fragment_shader(fragment_name: &str, device: &wgpu::Device) -> wgpu::ShaderModule {
        let fs_src = fs::read_to_string(fragment_name).expect("Could not load fragment shader");
        let fs_spirv = glsl_to_spirv::compile(&fs_src, glsl_to_spirv::ShaderType::Fragment).unwrap();
        let fs_data = wgpu::read_spirv(fs_spirv).unwrap();

        device.create_shader_module(&fs_data)
    }
    pub fn create_vertex_shader(vertex_name: &str, device: &wgpu::Device) -> wgpu::ShaderModule {
        let vs_src = fs::read_to_string(vertex_name).expect("Could not load fragment shader");
        let vs_spirv = glsl_to_spirv::compile(&vs_src, glsl_to_spirv::ShaderType::Vertex).unwrap();
        let vs_data = wgpu::read_spirv(vs_spirv).unwrap();
        
        device.create_shader_module(&vs_data)
    }
}

pub trait Drawable<'a> {
    fn get_vertex_information(&'a self) -> (&'a[u16], &'a[Vertex]);
    fn get_texture_name(&self) -> Option<String>;
}

pub struct DrawableFrame {
    // There must always be exactly one Option::Some
    // TODO: Convert to Union?
    texture_view: Option<Box<wgpu::TextureView>>,
    sc_output: Option<wgpu::SwapChainOutput>,
}

#[allow(dead_code)]
impl DrawableFrame {
    pub fn from_texture_view(texture_view: Box<wgpu::TextureView>) -> Self {
        Self {
            texture_view: Some(texture_view),
            sc_output: None,
        }
    }

    pub fn from_sc_output(sc_output: wgpu::SwapChainOutput) -> Self {
        Self {
            texture_view: None,
            sc_output: Some(sc_output),
        }
    }

    fn get_frame(&self) -> &wgpu::TextureView {
        match &self.texture_view {
            Some(boxed_view) => &*boxed_view,
            None => {
                match &self.sc_output {
                    Some(sc_output) => &sc_output.view,
                    None => panic!(),
                }
            }
        }
    }
}

pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,

    triangle_render_pipeline: wgpu::RenderPipeline,

    vertex_buffer: wgpu::Buffer,
    vertex_buffer_write_mapping: wgpu::BufferWriteMapping,
    index_buffer: wgpu::Buffer,
    index_buffer_write_mapping: wgpu::BufferWriteMapping,

    max_vertices: u32,
    max_indices: u32,
    num_vertices: u32,
    num_indices: u32,
    
    loaded_textures: HashMap<String, Box<Texture>>,
    draw_call_textures: [Option<String>; MAX_TEXTURES_PER_DRAW_CALL],
    
    texture_sampler_bind_group: wgpu::BindGroup,
    texture_bind_group_layout: wgpu::BindGroupLayout,

    uniforms: Uniforms,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,

    nr_draws_this_frame: u32,
    frame: Option<DrawableFrame>,

    clear_color: wgpu::Color,
}

impl Renderer {
    pub async fn new(max_vertices: u32, max_indices: u32, format: wgpu::TextureFormat) -> Self {
        let adapter = wgpu::Adapter::request(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: None,//Some(&surface),
            },
            wgpu::BackendBit::PRIMARY,
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                extensions: wgpu::Extensions {
                    anisotropic_filtering: false,
                },
                limits: Default::default(),
            }
        ).await;

        let placeholder_texture_bytes = include_bytes!("textures/placeholder_texture.png");
        let (placeholder_texture, cmd_buffer) = Texture::from_bytes(
            &device,
            placeholder_texture_bytes,
            PLACEHOLDER_TEXTURE_NAME,
        ).unwrap();

        queue.submit(&[cmd_buffer]);

        macro_rules! create_tex_sampler_desc {
            ($address_mode:expr, $filter_mode:expr) => {
                &wgpu::SamplerDescriptor {
                    address_mode_u: $address_mode,
                    address_mode_v: $address_mode,
                    address_mode_w: $address_mode,
                    mag_filter: $filter_mode,
                    min_filter: $filter_mode,
                    mipmap_filter: $filter_mode,
                    lod_min_clamp: -100.0,
                    lod_max_clamp: 100.0,
                    compare: wgpu::CompareFunction::Always,
                }
            }
        }

        let texture_samplers = [
            device.create_sampler(create_tex_sampler_desc!(wgpu::AddressMode::ClampToEdge,  wgpu::FilterMode::Nearest)), // Binding 0
            device.create_sampler(create_tex_sampler_desc!(wgpu::AddressMode::Repeat,       wgpu::FilterMode::Nearest)),
            device.create_sampler(create_tex_sampler_desc!(wgpu::AddressMode::MirrorRepeat, wgpu::FilterMode::Nearest)),
            device.create_sampler(create_tex_sampler_desc!(wgpu::AddressMode::ClampToEdge,  wgpu::FilterMode::Linear)),
            device.create_sampler(create_tex_sampler_desc!(wgpu::AddressMode::Repeat,       wgpu::FilterMode::Linear)),
            device.create_sampler(create_tex_sampler_desc!(wgpu::AddressMode::MirrorRepeat, wgpu::FilterMode::Linear)), // Binding 5
        ];

        macro_rules! create_bind_group_layout_entry {
            ($binding:expr) => {
                wgpu::BindGroupLayoutEntry {
                    binding: $binding,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler {
                        comparison: false,
                    },
                }
            }
        }

        let texture_sampler_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                bindings: &[
                    create_bind_group_layout_entry!(0),
                    create_bind_group_layout_entry!(1),
                    create_bind_group_layout_entry!(2),
                    create_bind_group_layout_entry!(3),
                    create_bind_group_layout_entry!(4),
                    create_bind_group_layout_entry!(5),
                ],
                label: Some("texture_bind_group_layout"),
            }
        );

        macro_rules! create_sampler_bind_group_binding {
            ($binding:expr) => {
                wgpu::Binding {
                    binding: $binding,
                    resource: wgpu::BindingResource::Sampler(&texture_samplers[$binding]),
                }
            }
        }

        let texture_sampler_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &texture_sampler_bind_group_layout,
                bindings: &[
                    create_sampler_bind_group_binding!(0),
                    create_sampler_bind_group_binding!(1),
                    create_sampler_bind_group_binding!(2),
                    create_sampler_bind_group_binding!(3),
                    create_sampler_bind_group_binding!(4),
                    create_sampler_bind_group_binding!(5),
                ],
                label: Some("texture_samplers_bind_group"),
            }
        );

        macro_rules! create_tex_desc {
            ($binding: expr) => {
                wgpu::BindGroupLayoutEntry {
                    binding: $binding,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::SampledTexture {
                        multisampled: false,
                        dimension: wgpu::TextureViewDimension::D2,
                        component_type: wgpu::TextureComponentType::Uint,
                    },
                }
            }
        }

        let texture_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                bindings: &[
                    create_tex_desc!(0),
                    create_tex_desc!(1),
                    create_tex_desc!(2),
                    create_tex_desc!(3),
                    create_tex_desc!(4),
                    create_tex_desc!(5),
                    create_tex_desc!(6),
                    create_tex_desc!(7),
                    create_tex_desc!(8),
                    create_tex_desc!(9),
                    ],
                label: Some("texture_bind_group_layout"),
            }
        );
        // Texture bind group will be created each frame

        let mut loaded_textures = HashMap::new();
        loaded_textures.insert(String::from(PLACEHOLDER_TEXTURE_NAME), placeholder_texture);

        let draw_call_textures = [None, None, None, None, None, None, None, None, None, None];

        let vs_module = shader::create_vertex_shader("src/shaders/shader.vs", &device);
        let fs_module = shader::create_fragment_shader("src/shaders/shader.fs", &device);

        let uniforms = Uniforms::new();

        let uniform_buffer = device.create_buffer_with_data(
            bytemuck::cast_slice(&[uniforms]),
            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        );

        let uniform_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                bindings: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::VERTEX,
                        ty: wgpu::BindingType::UniformBuffer {
                            dynamic: false,
                        },
                    },
                ],
                label: Some("uniform_bind_group_layout"),
            }
        );

        let uniform_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &uniform_bind_group_layout,
                bindings: &[
                    wgpu::Binding {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer {
                            buffer: &uniform_buffer,
                            range: 0..std::mem::size_of_val(&uniforms) as wgpu::BufferAddress,
                        },
                    },
                ],
                label: Some("uniform_bind_group"),
            },
        );
        
        let render_pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&uniform_bind_group_layout, &texture_sampler_bind_group_layout, &texture_bind_group_layout],
            }
        );

        let triangle_render_pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                layout: &render_pipeline_layout,
                vertex_stage: wgpu::ProgrammableStageDescriptor {
                    module: &vs_module,
                    entry_point: "main",
                },
                fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                    module: &fs_module,
                    entry_point: "main",
                }),
                rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: wgpu::CullMode::None,
                    depth_bias: 0,
                    depth_bias_slope_scale: 0.0,
                    depth_bias_clamp: 0.0,
                }),
                primitive_topology: wgpu::PrimitiveTopology::TriangleList,
                color_states: &[
                    wgpu::ColorStateDescriptor {
                        format: format,//sc_desc.format,
                        color_blend: wgpu::BlendDescriptor {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha_blend: wgpu::BlendDescriptor::REPLACE,
                        write_mask: wgpu::ColorWrite::ALL,
                    },
                ],
                depth_stencil_state: None,
                vertex_state: wgpu::VertexStateDescriptor {
                    index_format: wgpu::IndexFormat::Uint16,
                    vertex_buffers: &[
                        Vertex::desc(),
                    ],
                },
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            },
        );

        // let vertex_buffer = device.create_buffer_with_data(
        //     bytemuck::cast_slice(VERTICES),
        //     wgpu::BufferUsage::VERTEX,
        // );

        let vertex_buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("Vertex buffer"),
                size: (std::mem::size_of::<Vertex>() as u64 * max_vertices as u64) as wgpu::BufferAddress,
                usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::MAP_WRITE,
            }
        );
        let future_vertex_write_mapping = vertex_buffer.map_write(
            0,
            (std::mem::size_of::<Vertex>() as u64 * max_vertices as u64) as wgpu::BufferAddress,
        );
        
        // let index_buffer = device.create_buffer_with_data(
        //     bytemuck::cast_slice(INDICES),
        //     wgpu::BufferUsage::INDEX,
        // );
            
        let index_buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("Index buffer"),
                size: (std::mem::size_of::<u16>() as u64 * max_indices as u64) as wgpu::BufferAddress,
                usage: wgpu::BufferUsage::INDEX | wgpu::BufferUsage::MAP_WRITE,
            }
        );
        let future_index_write_mapping = index_buffer.map_write(
            0,
            (std::mem::size_of::<u16>() as u64 * max_indices as u64) as wgpu::BufferAddress,
        );
        
        device.poll(wgpu::Maintain::Wait);
        let vertex_buffer_write_mapping = future_vertex_write_mapping.await.unwrap();
        let index_buffer_write_mapping = future_index_write_mapping.await.unwrap();
            
        Self {
            device,
            queue,

            triangle_render_pipeline,

            vertex_buffer,
            vertex_buffer_write_mapping,
            index_buffer,
            index_buffer_write_mapping,

            max_vertices,
            max_indices,
            num_vertices: 0,
            num_indices: 0,

            loaded_textures,
            draw_call_textures,

            texture_sampler_bind_group,
            texture_bind_group_layout,

            uniforms,
            uniform_buffer,
            uniform_bind_group,

            nr_draws_this_frame: 0,
            frame: None,

            clear_color: wgpu::Color{r:0.1, g:0.2, b:0.3, a:0.0},
        }
    }

    pub fn get_device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn update(&mut self, camera: &Camera) {
        self.device.poll(wgpu::Maintain::Poll);

        self.uniforms.set_camera(camera);

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("update encoder"),
            },
        );
        let staging_buffer = self.device.create_buffer_with_data(
            bytemuck::cast_slice(&[self.uniforms]),
            wgpu::BufferUsage::COPY_SRC,
        );
        encoder.copy_buffer_to_buffer(&staging_buffer, 0, &self.uniform_buffer, 0, std::mem::size_of::<Uniforms>() as wgpu::BufferAddress);
        self.queue.submit(&[encoder.finish()]);
    }

    pub fn begin_render(&mut self, frame: DrawableFrame) {
        match self.frame {
            Some(_) => panic!("previous render has not ended"),
            None => self.frame = Some(frame),
        }
        self.nr_draws_this_frame = 0;
    }

    fn render(&mut self) {
        let frame = match &self.frame {
            Some(x) => x.get_frame(),
            None => panic!(),
        };

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            }
        );

        self.vertex_buffer.unmap();
        self.index_buffer.unmap();

        macro_rules! create_texture_bind_group_binding {
            ($binding:expr) => {{
                let texture = match &self.draw_call_textures[$binding] {
                    Some(name) => {
                        match self.loaded_textures.get(name) {
                            Some(tex) => tex,
                            None => self.loaded_textures.get(PLACEHOLDER_TEXTURE_NAME).unwrap()
                        }
                    }
                    None => {
                        self.loaded_textures.get(PLACEHOLDER_TEXTURE_NAME).unwrap()
                    }
                };
                wgpu::Binding {
                    binding: $binding,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                }
            }}
        }

        let texture_bind_group = self.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &self.texture_bind_group_layout,
                bindings: &[
                    create_texture_bind_group_binding!(0),
                    create_texture_bind_group_binding!(1),
                    create_texture_bind_group_binding!(2),
                    create_texture_bind_group_binding!(3),
                    create_texture_bind_group_binding!(4),
                    create_texture_bind_group_binding!(5),
                    create_texture_bind_group_binding!(6),
                    create_texture_bind_group_binding!(7),
                    create_texture_bind_group_binding!(8),
                    create_texture_bind_group_binding!(9),
                ],
                label: Some("texture_bind_group"),
            },
        );
        // println!("{:?}", self.draw_call_textures);
        self.draw_call_textures = [None, None, None, None, None, None, None, None, None, None];

        let operation = if self.nr_draws_this_frame == 0 {
            wgpu::LoadOp::Clear
        } else {
            wgpu::LoadOp::Load
        };

        let mut render_pass = encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: frame,
                        resolve_target: None,
                        load_op: operation,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: self.clear_color,
                    },
                ],
                depth_stencil_attachment: None,
            },
        );

        render_pass.set_pipeline(&self.triangle_render_pipeline);
        render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
        render_pass.set_bind_group(1, &self.texture_sampler_bind_group, &[]);
        render_pass.set_bind_group(2, &texture_bind_group, &[]);
        
        render_pass.set_vertex_buffer(0, &self.vertex_buffer, 0, 0);
        render_pass.set_index_buffer(&self.index_buffer, 0, 0);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        self.num_vertices = 0;
        self.num_indices = 0;

        drop(render_pass);

        self.queue.submit(
            &[encoder.finish()]
        );

        let future_vertex_write_mapping = self.vertex_buffer.map_write(
            0,
            (std::mem::size_of::<Vertex>() as u64 * self.max_vertices as u64) as wgpu::BufferAddress,
        );

        let future_index_write_mapping = self.index_buffer.map_write(
            0,
            (std::mem::size_of::<u16>() as u64 * self.max_indices as u64) as wgpu::BufferAddress,
        );
        
        self.device.poll(wgpu::Maintain::Wait);
        use futures::executor::block_on;
        self.vertex_buffer_write_mapping = block_on(future_vertex_write_mapping).unwrap();
        self.index_buffer_write_mapping = block_on(future_index_write_mapping).unwrap();

        self.nr_draws_this_frame += 1;
    }

    pub fn end_render(&mut self) {
        self.render();
        self.frame = None;
    }

    pub fn load_texture(&mut self, bytes:&[u8], label: String) -> bool {
        let (texture, cmd_buffer) = Texture::from_bytes(&self.device, bytes, &label).unwrap();
        self.queue.submit(&[cmd_buffer]);

        let entry = self.loaded_textures.entry(label);
        match entry {
            hash_map::Entry::Occupied(..) => false,
            hash_map::Entry::Vacant(..) => {
                entry.or_insert(texture);
                true
            },
        }
    }

    fn add_to_index_buffer(&mut self, indices: &[u16]) {
        let index_buffer_data = self.index_buffer_write_mapping.as_slice();
        let range = 
            (self.num_indices as usize * std::mem::size_of::<u16>())..
            ((self.num_indices as usize + indices.len())*std::mem::size_of::<u16>());
        index_buffer_data[range].copy_from_slice(bytemuck::cast_slice(indices));
        self.num_indices += indices.len() as u32;
    }

    fn add_to_vertex_buffer(&mut self, vertices: &[Vertex]) {
        let vertex_buffer_data = self.vertex_buffer_write_mapping.as_slice();
        let range = 
            (self.num_vertices as usize * std::mem::size_of::<Vertex>())..
            ((self.num_vertices as usize + vertices.len()) * std::mem::size_of::<Vertex>());
        vertex_buffer_data[range].copy_from_slice(bytemuck::cast_slice(vertices));
        self.num_vertices += vertices.len() as u32;
    }

    pub fn draw<'a, T:Drawable<'a>>(&mut self, shape: &'a T, transformation: Option<&UsableTransform>) {
        let (shape_indices, shape_vertices) = shape.get_vertex_information::<>();

        if shape_indices.len() > (self.max_indices-self.num_indices) as usize {
            if shape_indices.len() > self.max_indices as usize {
                panic!("shape has more indices than the renderer's max_indices");
            }
            self.render();
        }
        if shape_vertices.len() > (self.max_vertices-self.num_vertices) as usize {
            if shape_vertices.len() > self.max_vertices as usize {
                panic!("shape has more vertices than the renderer's max_vertices");
            }
            self.render();
        }

        let mut indices: Vec<u16> = shape_indices.to_vec();
        for index in &mut indices {
            *index += self.num_vertices as u16;
        }

        let texture_name = shape.get_texture_name();
        
        match texture_name {
            Some(_) => {
                let mut current_texture_binding = 0;
                for i in 0..self.draw_call_textures.len() {
                    if self.draw_call_textures[i] == None {
                        self.draw_call_textures[i] = texture_name;
                        current_texture_binding = i;
                        break;
                    } else if self.draw_call_textures[i] == texture_name {
                        current_texture_binding = i;
                        break;
                    }
                    // Out of texture binding locations
                    // Render the current information, then add the texture to the new empty array
                    if i == self.draw_call_textures.len()-1 {
                        self.render();
                        // Put the texture in the first slot int he textures array
                        // The array should be cleared in render(); if it isn't, something has gone terribly wrong
                        assert_eq!(self.draw_call_textures[0], None);
                        self.draw_call_textures[0] = texture_name;
                        current_texture_binding = 0;
                        // All the indices in shape_indices were incremented with the old number of vertices so it needs to be remade
                        // Because the new number of indices should start at 0, no need to add self.num_vertices
                        indices = shape_indices.to_vec();
                        break; // Logically unnecessary, but the borrow checker will complain without it
                    }
                }
                let mut vertices: Vec<Vertex> = shape_vertices.to_vec();
                match transformation {
                    Some(transf) => {
                        let transformation_matrix = transf.get_transformation_matrix();
                        for vertex in &mut vertices {
                            vertex.texture_binding = current_texture_binding as i32;
                            UsableTransform::transform_point_with_matrix(&mut vertex.position, &transformation_matrix)
                        }
                    }
                    None => {
                        for vertex in &mut vertices {
                            vertex.texture_binding = current_texture_binding as i32;
                        }
                    }
                }
                self.add_to_vertex_buffer(&vertices);
                self.add_to_index_buffer(&indices);
            },
            None => {
                match transformation {
                    Some(transf) => {
                        let mut vertices: Vec<Vertex> = shape_vertices.to_vec();
                        let transformation_matrix = transf.get_transformation_matrix();
                        for vertex in &mut vertices {
                            UsableTransform::transform_point_with_matrix(&mut vertex.position, &transformation_matrix)
                        }
                        self.add_to_vertex_buffer(&vertices);
                        self.add_to_index_buffer(&indices);
                    }
                    None => {
                        self.add_to_vertex_buffer(shape_vertices);
                        self.add_to_index_buffer(&indices);
                    }
                }
                
            }
        }
    }
}