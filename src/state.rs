use std::{f32, sync::Arc, time::Instant};

use bytemuck::cast_slice;
use rand::{Rng, rng};
use wgpu::{
    Adapter, Buffer, Device, Instance, Queue, RenderPipeline, Surface, SurfaceCapabilities,
    SurfaceConfiguration, SurfaceError, VertexAttribute, VertexBufferLayout, util::DeviceExt,
};
use winit::{dpi::PhysicalSize, window::Window};

use pollster::FutureExt;

use crate::{
    configuration::{DIVISION, DynInfo, OBJ_COUNT, StaticInfo, TIME_STEP},
    util::create_annulus_vertices,
};

pub struct State<'a> {
    surface: Surface<'a>,
    device: Device,
    queue: Queue,
    size: PhysicalSize<u32>,
    config: SurfaceConfiguration,
    window: Arc<Window>,
    render_pipeline: RenderPipeline,
    scales: [f32; OBJ_COUNT],
    velocities: [[f32; 2]; OBJ_COUNT],
    dyn_info: DynInfo,
    static_buffer: Buffer,
    dyn_buffer: Buffer,
    vertex_buffer: Buffer,
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
        let vertex_info =
            create_annulus_vertices::<DIVISION>(0.5, 0.25, 0.0, f32::consts::PI * 2.0);

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: cast_slice(&vertex_info),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let mut rng = rng();

        // init buffer data
        // f32 is required here
        let mut static_info: StaticInfo = [[1.0; 4]; OBJ_COUNT];
        let mut dyn_info: DynInfo = [[1.0; 4]; OBJ_COUNT];
        let mut scales: [f32; OBJ_COUNT] = [1.0; OBJ_COUNT];
        let mut velocities: [[f32; 2]; OBJ_COUNT] = [[0.0, 0.0]; OBJ_COUNT];
        for i in 0..OBJ_COUNT {
            static_info[i] = [
                rng.random(),
                rng.random(),
                rng.random(),
                1.0, // alpha value will be neglected if "alpha_mode" is set to "opaque"
            ];
            dyn_info[i] = [
                rng.random_range(-1.0..1.0),
                rng.random_range(-1.0..1.0),
                0.5,
                0.5,
            ];
            scales[i] = rng.random::<f32>();
            velocities[i] = [rng.random_range(0.1..0.2), rng.random_range(0.1..0.2)];
        }

        let static_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: cast_slice(&static_info),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let dyn_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: cast_slice(&dyn_info),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
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
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs"),
                buffers: &[
                    VertexBufferLayout {
                        array_stride: 2 * 4,
                        attributes: &[VertexAttribute {
                            shader_location: 0,
                            offset: 0,
                            format: wgpu::VertexFormat::Float32x2,
                        }],
                        step_mode: wgpu::VertexStepMode::Vertex,
                    },
                    VertexBufferLayout {
                        array_stride: 4 * 4,
                        attributes: &[VertexAttribute {
                            shader_location: 1,
                            offset: 0,
                            format: wgpu::VertexFormat::Float32x4,
                        }],
                        step_mode: wgpu::VertexStepMode::Instance,
                    },
                    VertexBufferLayout {
                        array_stride: 4 * 4,
                        attributes: &[
                            VertexAttribute {
                                shader_location: 2,
                                offset: 0,
                                format: wgpu::VertexFormat::Float32x2,
                            },
                            VertexAttribute {
                                shader_location: 3,
                                offset: 2 * 4,
                                format: wgpu::VertexFormat::Float32x2,
                            },
                        ],
                        step_mode: wgpu::VertexStepMode::Instance,
                    },
                ],
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
            scales,
            velocities,
            vertex_buffer,
            static_buffer,
            dyn_info,
            dyn_buffer,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.size = new_size;
        self.config.width = self.size.width;
        self.config.height = self.size.height;
        self.surface.configure(&self.device, &self.config);
        let ratio = (self.config.width as f32) / (self.config.height as f32);
        //update dyn_buffer
        for i in 0..OBJ_COUNT {
            self.dyn_info[i] = [
                self.dyn_info[i][0],
                self.dyn_info[i][1],
                self.scales[i] / ratio,
                self.scales[i],
            ]
        }
        self.queue
            .write_buffer(&self.dyn_buffer, 0, cast_slice(&self.dyn_info));
    }

    fn update_location_velocity(&mut self) {
        for i in 0..OBJ_COUNT {
            let (mut new_vx, mut new_vy) = (self.velocities[i][0], self.velocities[i][1]);
            let mut new_x = new_vx * TIME_STEP + &self.dyn_info[i][0];
            let mut new_y = new_vy * TIME_STEP + &self.dyn_info[i][1];
            match new_x {
                x if x >= 1.0 => {
                    new_x = 1.0;
                    new_vx = -new_vx;
                }
                x if x <= -1.0 => {
                    new_x = -1.0;
                    new_vx = -new_vx;
                }
                _ => {}
            }
            match new_y {
                y if y >= 1.0 => {
                    new_y = 1.0;
                    new_vy = -new_vy;
                }
                y if y <= -1.0 => {
                    new_y = -1.0;
                    new_vy = -new_vy;
                }
                _ => {}
            }
            self.dyn_info[i][0] = new_x;
            self.dyn_info[i][1] = new_y;
            self.velocities[i][0] = new_vx;
            self.velocities[i][1] = new_vy;
        }
        self.queue
            .write_buffer(&self.dyn_buffer, 0, cast_slice(&self.dyn_info));
    }

    //FIXME: render takes ~100ms when obj_count ~10000
    pub fn render(&mut self) -> Result<(), SurfaceError> {
        let render_start = Instant::now();
        self.update_location_velocity();
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&Default::default());

        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
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
            render_pass.set_vertex_buffer(1, self.static_buffer.slice(..));
            render_pass.set_vertex_buffer(2, self.dyn_buffer.slice(..));
            render_pass.draw(0..(DIVISION as u32) * 2 * 3, 0..OBJ_COUNT as u32);
        }

        self.queue.submit(Some(encoder.finish()));
        output.present();
        let render_end = Instant::now();
        println!("render: {:?}", render_end.duration_since(render_start));
        Ok(())
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}
