use std::borrow::Cow;
use std::collections::HashMap;
use crate::core::resource::{ResourceId, ResourcePools};
use crate::materials::material::{Material, Side};
use crate::objects::node::Node;

pub struct WGPURenderPipeline {
    pipeline: wgpu::RenderPipeline,
}

impl WGPURenderPipeline {
    fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        bind_group_layout: &wgpu::BindGroupLayout,
        shader_code: &str,
        sample_count: u32,
        side: &Side,
    ) -> Self {
        // For debug
        //println!("{}", shader_code);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(shader_code)),
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[bind_group_layout],
            push_constant_ranges: &[],
        });

        // TODO 根据 Geometry 动态构建
        let vertex_buffers = [
            // position
            wgpu::VertexBufferLayout {
                array_stride: 3 * 4,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &[
                    wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x3,
                        offset: 0,
                        shader_location: 0,
                    }
                ],
            },
            // normal
            wgpu::VertexBufferLayout {
                array_stride: 3 * 4,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &[
                    wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x3,
                        offset: 0,
                        shader_location: 1,
                    },
                ],
            },
            // uv
            wgpu::VertexBufferLayout {
                array_stride: 2 * 4,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &[
                    wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x2,
                        offset: 0,
                        shader_location: 2,
                    },
                ],
            },
        ];

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                compilation_options: Default::default(),
                buffers: &vertex_buffers,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                // @TODO: Color management
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            // Backface culling
            // @TODO: Should be configurable
            multiview: None,
            primitive: wgpu::PrimitiveState {
                cull_mode: match side {
                    Side::BackSide |
                    Side::FrontSide => Some(wgpu::Face::Back),
                    Side::DoubleSide => None,
                },
                front_face: match side {
                    Side::BackSide => wgpu::FrontFace::Cw,
                    Side::DoubleSide |
                    Side::FrontSide => wgpu::FrontFace::Ccw,
                },
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                bias: wgpu::DepthBiasState::default(),
                depth_compare: wgpu::CompareFunction::LessEqual,
                depth_write_enabled: true,
                format: wgpu::TextureFormat::Depth24PlusStencil8,
                stencil: wgpu::StencilState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: sample_count,
                ..Default::default()
            },
            cache: None,
        });

        WGPURenderPipeline {
            pipeline
        }
    }
}

pub struct WGPURenderPipelines {
    pipelines: HashMap::<ResourceId<Node>, WGPURenderPipeline>,
    config: wgpu::SurfaceConfiguration,
}

impl WGPURenderPipelines {
    pub fn new(config: wgpu::SurfaceConfiguration) -> Self {
        Self {
            pipelines: HashMap::new(),
            config
        }
    }

    pub fn borrow(&self, node: &ResourceId<Node>) -> Option<&wgpu::RenderPipeline> {
        if let Some(pipeline) = &self.pipelines.get(node) {
            Some(&pipeline.pipeline)
        } else {
            None
        }
    }

    pub fn update(
        &mut self,
        device: &wgpu::Device,
        pools: &ResourcePools,
        node: &ResourceId<Node>,
        material: &Material,
        bind_group_layout: &wgpu::BindGroupLayout,
        sample_count: u32,
    ) {
        if !self.pipelines.contains_key(&node) {
            self.pipelines.insert(
                *node,
                WGPURenderPipeline::new(
                    device,
                    &self.config,
                    bind_group_layout,
                    &material.build_shader_code(),
                    sample_count,
                    material.borrow_side(),
                ),
            );
        }
    }
}
