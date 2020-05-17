use winit::event_loop::EventLoop;

pub struct Window {
    pub winit_window: winit::window::Window,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
}

impl Window {
    pub fn new<T>(event_loop: &EventLoop<T>, format: wgpu::TextureFormat, device: &wgpu::Device) -> Self {
        let winit_window = winit::window::WindowBuilder::new().build(event_loop).unwrap();
        let size = winit_window.inner_size();

        let surface = wgpu::Surface::create(&winit_window);

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        Self {
            winit_window,
            size,
            surface,
            sc_desc,
            swap_chain,
        }
    }

    pub fn get_next_frame(&mut self) -> wgpu::SwapChainOutput {
        self.swap_chain.get_next_texture().unwrap()
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>, device: &wgpu::Device) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    pub fn get_aspect_ratio(&self) -> f32 {
        self.size.width as f32 / self.size.height as f32
    }
}