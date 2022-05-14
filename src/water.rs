use cgmath::{Matrix4, SquareMatrix};

use crate::{asset_manager, mesh::Mesh, model::Model, window_handler::GlContext};

pub struct Water {
    model: Model,
}

impl Water {
    pub fn new(gl: &GlContext, size: (usize, usize), scale: f32, height: f32) -> Self {
        let mut vertices = vec![
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 1.0],
        ];

        let base_height = 40.0;
        for vertex in vertices.iter_mut() {
            vertex[0] = vertex[0] * scale * size.0 as f32;
            vertex[1] = (base_height + vertex[1]) * height;
            vertex[2] = vertex[2] * scale * size.1 as f32;
        }

        let texies = vec![[0.0, 1.0], [1.0, 1.0], [0.0, 0.0], [0.0, 1.0]];

        let normals = vec![
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
        ];

        let indices = vec![0, 1, 2, 1, 3, 2];

        let program =
            asset_manager::load_shaders(gl, "assets/water/water.vert", "assets/water/water.frag");
        let textures = asset_manager::load_textures(gl, &["assets/water/water.jpg"]);

        Self {
            model: Model::new(
                gl,
                Mesh {
                    vertices,
                    texies,
                    normals,
                    indices,
                },
                program,
                textures,
            ),
        }
    }

    pub fn render(&self, gl: &GlContext, time: &f32, cam_per: &[f32; 16]) {
        let matrix = Matrix4::identity();
        self.model.render(gl, matrix, time, cam_per);
    }
}
