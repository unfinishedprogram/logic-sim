use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupLayout, Buffer, BufferUsages, Device, Queue,
};

use crate::util::bounds::Bounds;

use super::bindable::Bindable;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Camera {
    pub center: Vec2,
    pub size: Vec2,
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

    pub fn bounds(&self) -> Bounds {
        Bounds::from_center_and_size(self.center, self.size * 2.0)
    }
}

pub struct CameraUniform {
    bind_group: BindGroup,
    bind_group_layout: BindGroupLayout,
    buffer: Buffer,
}

impl CameraUniform {
    const BG_LAYOUT_DESCRIPTOR: wgpu::BindGroupLayoutDescriptor<'static> =
        wgpu::BindGroupLayoutDescriptor {
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
        };

    const BUF_INIT_DESC: BufferInitDescriptor<'static> = BufferInitDescriptor {
        label: Some("Camera Buffer"),
        contents: &[0; size_of::<Camera>()],
        // This ugly bitflag -> u32 -> bitflag conversion is needed for this to be const
        usage: BufferUsages::from_bits_retain(
            BufferUsages::UNIFORM.bits() | BufferUsages::COPY_DST.bits(),
        ),
    };

    pub fn create(device: &Device) -> Self {
        let bind_group_layout = device.create_bind_group_layout(&Self::BG_LAYOUT_DESCRIPTOR);
        let buffer = device.create_buffer_init(&Self::BUF_INIT_DESC);

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

impl Bindable for CameraUniform {
    fn bind_group_layout(&self) -> &BindGroupLayout {
        &self.bind_group_layout
    }

    fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}
