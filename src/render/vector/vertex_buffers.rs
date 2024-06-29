use lyon::tessellation::VertexBuffers;

pub trait VertexBufferUtils {
    fn extend(&mut self, other: Self);

    #[cfg(feature = "rayon")]
    fn join(buffers: Vec<Self>) -> Self
    where
        Self: Sized;
}

impl<OutVert> VertexBufferUtils for VertexBuffers<OutVert, u32> {
    fn extend(&mut self, other: Self) {
        let index_offset = self.vertices.len() as u32;

        self.vertices.extend(other.vertices);
        self.indices
            .extend(other.indices.into_iter().map(|index| index + index_offset));
    }

    #[cfg(feature = "rayon")]
    fn join(buffers: Vec<Self>) -> Self
    where
        Self: Sized,
    {
        let (num_vertices, num_indices) = buffers.iter().fold((0, 0), |(v, i), buffer| {
            (v + buffer.vertices.len(), i + buffer.indices.len())
        });

        buffers.into_iter().fold(
            VertexBuffers::with_capacity(num_vertices, num_indices),
            |mut acc, buffer| {
                acc.extend(buffer);
                acc
            },
        )
    }
}
