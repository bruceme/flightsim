pub struct Mesh {
    pub vertices: Vec<[f32; 3]>,
    pub texies: Vec<[f32; 2]>,
    pub normals: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn new(
        vertices: Vec<[f32; 3]>,
        texies: Vec<[f32; 2]>,
        normals: Vec<[f32; 3]>,
        indices: Vec<u32>,
    ) -> Self {
        Self {
            vertices,
            texies,
            normals,
            indices,
        }
    }
}
