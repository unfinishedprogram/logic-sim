use super::{quad::Quad, vertex::Vertex};

pub struct Scene {
    pub triangles: Vec<[Vertex; 3]>,
}

impl Scene {
    pub fn as_vertex_buffer(&self) -> &[u8] {
        bytemuck::cast_slice(&self.triangles)
    }

    pub fn add_mesh(&mut self, mesh: &[[Vertex; 3]]) {
        self.triangles.extend_from_slice(mesh);
    }

    pub fn size(&self) -> u32 {
        self.triangles.len() as u32 * 3
    }

    pub fn new() -> Self {
        let mut scene = Scene {
            triangles: Vec::new(),
        };

        let mut mesh = Vec::new();

        for x in 0..10 {
            for y in 0..10 {
                let my_quad = Quad::new(
                    Vertex::new(x as f32, y as f32),
                    Vertex::new(x as f32 + 1.0, y as f32 + 1.0),
                );

                mesh.extend_from_slice(&my_quad.triangles());
            }
        }

        scene.add_mesh(&mesh);
        println!("Scene has {} triangles", scene.triangles.len());
        scene
    }
}
