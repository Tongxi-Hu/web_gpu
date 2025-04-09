use std::{f32, iter::once, sync::Arc};

use bytemuck::cast_slice;
use rand::Rng;
use wgpu::{
    Adapter, BindGroup, Buffer, Device, Instance, Queue, RenderPipeline, Surface,
    SurfaceCapabilities, SurfaceConfiguration, SurfaceError, VertexAttribute, VertexBufferLayout,
    util::DeviceExt,
};
use winit::{dpi::PhysicalSize, window::Window};

use pollster::FutureExt;

pub struct State<'a> {
    surface: Surface<'a>,
    device: Device,
    queue: Queue,
    size: PhysicalSize<u32>,
    config: SurfaceConfiguration,
    window: Arc<Window>,
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    bind_group_value: [f32; 12],
    bind_group_buffer: Buffer,
    bind_group: BindGroup,
    angle: f32,
}

impl<'a> State<'a> {
    fn create_surface_config(
        size: PhysicalSize<u32>,
        capabilities: SurfaceCapabilities,
    ) -> wgpu::SurfaceConfiguration {
        let surface_format = capabilities
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(capabilities.formats[0]);

        wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoNoVsync,
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        }
    }

    fn create_device(adapter: &Adapter) -> (Device, Queue) {
        adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: None,
                    memory_hints: Default::default(),
                },
                None,
            )
            .block_on()
            .unwrap()
    }

    fn create_adaptor(instance: Instance, surface: &Surface) -> Adapter {
        instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .block_on()
            .unwrap()
    }

    fn create_gpu_instance() -> Instance {
        Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        })
    }

    pub fn new(window: Window) -> Self {
        let window_arc = Arc::new(window);
        let size = window_arc.inner_size();
        let instance = Self::create_gpu_instance();
        let surface = instance.create_surface(window_arc.clone()).unwrap();
        let adapter = Self::create_adaptor(instance, &surface);
        let (device, queue) = Self::create_device(&adapter);
        let surface_caps = surface.get_capabilities(&adapter);
        let config = Self::create_surface_config(size, surface_caps);
        surface.configure(&device, &config);

        // vertex buffer
        let vertex_info: [f32; 6] = [0.0, 0.0, 50.0, 150.0, 100.0, 0.0];

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: cast_slice(&vertex_info),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let angle: f32 = 0.0;
        let mut rng = rand::rng();
        let bind_group_value: [f32; 12] = [
            rng.random_range(0.0..1.0),
            rng.random_range(0.0..1.0),
            rng.random_range(0.0..1.0),
            1.0, //color
            config.width as f32,
            config.height as f32, //resolution
            0.0,
            0.0, // translation
            angle.cos(),
            angle.sin(), //rotation
            0.0,
            0.0, //padding
        ];

        let bind_group_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: cast_slice(&bind_group_value),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: None,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: bind_group_buffer.as_entire_binding(),
            }],
            label: None,
        });

        //shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        //prepare render pipeline
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs"),
                buffers: &[VertexBufferLayout {
                    array_stride: 2 * 4,
                    attributes: &[VertexAttribute {
                        shader_location: 0,
                        offset: 0,
                        format: wgpu::VertexFormat::Float32x2,
                    }],
                    step_mode: wgpu::VertexStepMode::Vertex,
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        Self {
            surface,
            device,
            queue,
            size,
            config,
            window: window_arc,
            render_pipeline,
            vertex_buffer,
            bind_group_value,
            bind_group_buffer,
            bind_group,
            angle,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.size = new_size;
        self.config.width = self.size.width;
        self.config.height = self.size.height;
        self.surface.configure(&self.device, &self.config);
        self.bind_group_value[4] = self.config.width as f32;
        self.bind_group_value[5] = self.config.height as f32;
        self.queue.write_buffer(
            &self.bind_group_buffer,
            0,
            cast_slice(&self.bind_group_value),
        );
    }

    pub fn update(&mut self) {
        self.angle = self.angle + 2.0 * std::f32::consts::PI / 180.0;
        self.bind_group_value[6] = self.bind_group_value[6] + 2.0;
        self.bind_group_value[7] = self.bind_group_value[7] + 2.0;
        self.bind_group_value[8] = self.angle.cos();
        self.bind_group_value[9] = self.angle.sin();
        self.queue.write_buffer(
            &self.bind_group_buffer,
            0,
            cast_slice(&self.bind_group_value),
        );
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        self.update();
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&Default::default());
        let mut encoder = self.device.create_command_encoder(&Default::default());

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw(0..3, 0..1);
        drop(render_pass);
        self.queue.submit(once(encoder.finish()));
        output.present();
        Ok(())
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}
