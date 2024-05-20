pub trait Bindable {
    fn bind_group_layout(&self) -> &wgpu::BindGroupLayout;
    fn bind_group(&self) -> &wgpu::BindGroup;
}

pub struct BindList<'a> {
    pub bind_groups: Vec<&'a dyn Bindable>,
}

impl<'a> BindList<'a> {
    pub fn new() -> Self {
        Self {
            bind_groups: Vec::new(),
        }
    }

    pub fn push(&mut self, bindable: &'a dyn Bindable) {
        self.bind_groups.push(bindable);
    }

    pub fn bind_group_layouts(&self) -> Vec<&wgpu::BindGroupLayout> {
        self.bind_groups
            .iter()
            .map(|bindable| bindable.bind_group_layout())
            .collect()
    }

    pub fn bind_groups(&self) -> Vec<&wgpu::BindGroup> {
        self.bind_groups
            .iter()
            .map(|bindable| bindable.bind_group())
            .collect()
    }
}

pub trait BindTarget<'a> {
    fn set_bind_groups<'b>(&'b mut self, groups: &'a BindList<'a>)
    where
        'a: 'b;
}

impl<'pass, 'a> BindTarget<'a> for wgpu::RenderPass<'pass>
where
    'a: 'pass,
{
    fn set_bind_groups<'b>(&'b mut self, groups: &'a BindList<'a>)
    where
        'a: 'b,
    {
        for (i, bind_group) in groups.bind_groups().into_iter().enumerate() {
            self.set_bind_group(i as u32, bind_group, &[]);
        }
    }
}
