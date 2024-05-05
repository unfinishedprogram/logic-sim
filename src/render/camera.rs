use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use wgpu::{util::{BufferInitDescriptor, DeviceExt}, BindGroup, BindGroupLayout, Buffer, BufferUsages, Device, Queue};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct CameraUniform {
    pub center: Vec2,
    pub size: Vec2,
}

impl CameraUniform {
    pub fn translate(&mut self, translation: Vec2) {
        self.center += translation * 0.1;
    }
}

pub struct Camera {
    pub uniform: CameraUniform,
    bind_group: BindGroup,
    bind_group_layout: BindGroupLayout,
    buffer: Buffer,
}

impl Camera {
    pub fn translate(&mut self, translation: Vec2) {
        self.uniform.center += translation;
    }

    pub fn scale(&mut self, scale: Vec2) {
        self.uniform.size *= scale;
    }

    pub fn set_aspect(&mut self, ratio: f32, scale: f32) {
        self.uniform.size = Vec2::new(scale, scale / ratio);
    }

    pub fn screen_space_to_world_space(&self, screen_space: Vec2, screen_size: Vec2) -> Vec2 {
        (screen_space / screen_size - self.uniform.center) / self.uniform.size
    }

    pub fn create(device:&Device) -> Self {
        let uniform = CameraUniform {
            center: glam::Vec2::new(0.0, 0.0),
            size: glam::Vec2::new(10.0, 10.0),
        };

        let bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("Camera Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            },
        );

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }
            ],
            label: Some("camera_bind_group"),
        });

        Self {
            uniform,
            bind_group,
            bind_group_layout,
            buffer,
        }
    }

    pub fn bind_group_layout(&self) -> &BindGroupLayout {
        &self.bind_group_layout
    }
    pub fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }

    pub fn update(&self, queue: &Queue) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.uniform]));
    }
}