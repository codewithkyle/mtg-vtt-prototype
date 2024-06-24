use winit::window::Window;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pos: [f32; 3],
    tex_cords: [f32; 2],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        todo!("Write vertex buffer layout");
    }
}

pub struct Instance {
    pos: cgmath::Vector3<f32>,
    rotation: cgmath::Quaternion<f32>,
}

impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        todo!("Convert to raw instance");
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    model: [[f32; 4]; 4],
}

impl InstanceRaw {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        todo!("Write instance buffer layout");
    }
}

pub struct Renderer<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: &'a Window,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    diffuse_bind_group: wgpu::BindGroup,
    instances: Vec<Instance>,
    instance_buffer: wgpu::Buffer,
}

impl<'a> Renderer<'a> {
    pub async fn new(window: &'a Window) -> Renderer<'a> {
        todo!();
    }

    pub fn window(&self) -> &Window {
        todo!();
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        todo!();
    }

    pub fn update(&mut self) {
        todo!();
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        todo!();
    }
}
