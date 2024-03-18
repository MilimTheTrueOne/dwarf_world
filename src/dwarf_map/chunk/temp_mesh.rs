use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
};

pub struct TempMesh {
    uv: Vec<Vec2>,
    normals: Vec<Vec3>,
    vertices: Vec<Vec3>,
    indices: Vec<u32>,
}

impl TempMesh {
    pub fn new() -> Self {
        Self {
            uv: vec![],
            normals: vec![],
            vertices: vec![],
            indices: vec![],
        }
    }

    /// Function to add any mesh to this mesh.
    pub fn extend(&mut self, uv: &[Vec2], normals: &[Vec3], vertices: &[Vec3], indices: &[u32]) {
        // check that input data is valid,
        if uv.len() != normals.len() || uv.len() != vertices.len() {
            panic!("Attempt to insert invalid data into TempMesh!");
        }

        let old_length: u32 = self.uv.len() as u32;

        self.uv.extend(uv);
        self.normals.extend(normals);
        self.vertices.extend(vertices);
        self.indices.extend(indices.iter().map(|i| i + old_length));
    }

    pub fn into_mesh(self) -> Mesh {
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, self.uv)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, self.vertices)
        .with_inserted_indices(Indices::U32(self.indices))
    }
}
