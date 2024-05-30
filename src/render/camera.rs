use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupLayout, Buffer, BufferUsages, Device, Queue,
};

use super::bindable::Bindable;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Camera {
    pub center: Vec2,
    pub size: Vec2,
}

pub struct CameraBinding {
    bind_group: BindGroup,
    bind_group_layout: BindGroupLayout,
    buffer: Buffer,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            center: Vec2::ZERO,
            size: Vec2::ONE,
        }
    }

    pub fn translate(&mut self, translation: Vec2) {
        self.center += translation;
    }

    pub fn scale(&mut self, scale: Vec2) {
        self.size *= scale;
    }

    pub fn set_aspect(&mut self, ratio: f32, scale: f32) {
        self.size = Vec2::new(scale, scale / ratio);
    }

    pub fn set_aspect_ratio(&mut self, aspect: Vec2) {
        let magnitude = self.size.length();
        self.size = aspect.normalize() * magnitude;
    }

    pub fn top_left(&self) -> Vec2 {
        self.center - self.size
    }
}

impl CameraBinding {
    pub fn create(device: &Device) -> Self {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("camera_bind_group_layout"),
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
        });

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("camera_buffer"),
            contents: bytemuck::cast_slice(&[Camera::new()]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        Self {
            bind_group,
            bind_group_layout,
            buffer,
        }
    }

    pub fn update(&self, queue: &Queue, camera: &Camera) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[*camera]));
    }
}

impl Bindable for CameraBinding {
    fn bind_group_layout(&self) -> &BindGroupLayout {
        &self.bind_group_layout
    }

    fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }
}
