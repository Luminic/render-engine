use winit::event_loop::EventLoop;
use std::fmt;

#[derive(Debug)]
pub enum WindowError {
    UninitializedSwapChain,
    TimeOut,
}

impl fmt::Display for WindowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WindowError::UninitializedSwapChain => {
                write!(f, "Swap chain has not been initialized yet")
            },
            WindowError::TimeOut => {
                write!(f, "The GPU timed out when attempting to acquire the next texture or if a previous output is still alive.")
            }
        }
    }
}

impl From<wgpu::TimeOut> for WindowError {
    fn from(_: wgpu::TimeOut) -> WindowError {
        WindowError::TimeOut
    }
}

pub struct Window {
    pub winit_window: winit::window::Window,
    size: winit::dpi::PhysicalSize<u32>,
    format: wgpu::TextureFormat,
    surface: wgpu::Surface,
    sc_desc: Option<wgpu::SwapChainDescriptor>,
    swap_chain: Option<wgpu::SwapChain>,
}

impl Window {
    pub fn new<T>(event_loop: &EventLoop<T>, format: wgpu::TextureFormat) -> Self {
        let winit_window = winit::window::WindowBuilder::new().build(event_loop).unwrap();
        let size = winit_window.inner_size();

        let surface = wgpu::Surface::create(&winit_window);

        Self {
            winit_window,
            size,
            format,
            surface,
            sc_desc: None,
            swap_chain: None,
        }
    }

    pub fn init_swapchain(&mut self, device: &wgpu::Device) {
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: self.format,
            width: self.size.width,
            height: self.size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&self.surface, &sc_desc);

        self.sc_desc = Some(sc_desc);
        self.swap_chain = Some(swap_chain);
    }

    pub fn get_surface(&self) -> &wgpu::Surface {
        &self.surface
    }

    pub fn get_next_frame(&mut self) -> Result<wgpu::SwapChainOutput, WindowError> {
        match &mut self.swap_chain {
            Some(swap_chain) => {
                Ok(swap_chain.get_next_texture()?)
            },
            None => Err(WindowError::UninitializedSwapChain),
        }
    }

    // Will initialize swapchain if not already initialized
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>, device: &wgpu::Device) {
        self.size = new_size;
        match &mut self.sc_desc {
            Some(sc_desc) => {
                sc_desc.width = new_size.width;
                sc_desc.height = new_size.height;
                self.swap_chain = Some(device.create_swap_chain(&self.surface, sc_desc));
            },
            None => {
                self.init_swapchain(device);
            },
        }
    }

    pub fn get_aspect_ratio(&self) -> f32 {
        self.size.width as f32 / self.size.height as f32
    }
}