// use crate::gl::core::*;
// use crate::gltf::*;
// use crate::math::math::*;
// use crate::renderer::{buffer::*, vertex_array::*};
// use std::ffi;
// use std::sync::Arc;

// pub struct GltfRenderable {
//     pub meshes: Vec<RenderMesh>,
//     pub model: Mat4,
// }

// pub struct RenderMesh {
//     vao: VAO,
//     _vbo: VBO,
//     ebo: Option<EBO>,
//     vertex_count: i32,
//     has_indices: bool,
// }

// impl GltfRenderable {
//     pub fn from_gltf(gl: Arc<GL>, gltf_model: GltfModel) -> Self {
//         let meshes = gltf_model
//             .meshes
//             .into_iter()
//             .flat_map(|mesh| {
//                 mesh.primitives
//                     .into_iter()
//                     .map(|prim| Self::create_render_mesh(gl.clone(), prim))
//             })
//             .collect();

//         Self {
//             meshes,
//             model: Mat4::identity(),
//         }
//     }

//     pub fn load(gl: Arc<GL>, path: &str) -> Result<Self, String> {
//         let gltf_model = GltfModel::load(path)?;
//         Ok(Self::from_gltf(gl, gltf_model))
//     }

//     // fn create_render_mesh(gl: Arc<GL>, prim: Primitive) -> RenderMesh {
//     //     let vertex_count = prim.positions.len() / 3;

//     //     // Interleave vertex data: [pos.x, pos.y, pos.z, normal.x, normal.y, normal.z, uv.x, uv.y]
//     //     let mut vertices = Vec::with_capacity(vertex_count * 8);

//     //     for i in 0..vertex_count {
//     //         // Position
//     //         vertices.push(prim.positions[i * 3]);
//     //         vertices.push(prim.positions[i * 3 + 1]);
//     //         vertices.push(prim.positions[i * 3 + 2]);

//     //         // Normal (or default)
//     //         if let Some(ref normals) = prim.normals {
//     //             vertices.push(normals[i * 3]);
//     //             vertices.push(normals[i * 3 + 1]);
//     //             vertices.push(normals[i * 3 + 2]);
//     //         } else {
//     //             vertices.push(0.0);
//     //             vertices.push(1.0);
//     //             vertices.push(0.0);
//     //         }

//     //         // TexCoords (or default)
//     //         if let Some(ref uvs) = prim.tex_coords {
//     //             vertices.push(uvs[i * 2]);
//     //             vertices.push(uvs[i * 2 + 1]);
//     //         } else {
//     //             vertices.push(0.0);
//     //             vertices.push(0.0);
//     //         }
//     //     }

//     //     let vao = VAO::new(gl.clone());
//     //     vao.bind();

//     //     let vbo = VBO::new(gl.clone(), &vertices);

//     //     let stride = 8 * std::mem::size_of::<f32>() as i32;

//     //     // Position attribute (location = 0)
//     //     vao.attrib_pointer(
//     //         0,
//     //         3,
//     //         gl.data_type.float,
//     //         gl.boolean.false_,
//     //         stride,
//     //         std::ptr::null(),
//     //     );

//     //     // Normal attribute (location = 1)
//     //     vao.attrib_pointer(
//     //         1,
//     //         3,
//     //         gl.data_type.float,
//     //         gl.boolean.false_,
//     //         stride,
//     //         (3 * std::mem::size_of::<f32>()) as *const ffi::c_void,
//     //     );

//     //     // TexCoord attribute (location = 2)
//     //     vao.attrib_pointer(
//     //         2,
//     //         2,
//     //         gl.data_type.float,
//     //         gl.boolean.false_,
//     //         stride,
//     //         (6 * std::mem::size_of::<f32>()) as *const ffi::c_void,
//     //     );

//     //     let (ebo, has_indices) = if let Some(indices) = prim.indices {
//     //         (Some(EBO::new(gl.clone(), &indices)), true)
//     //     } else {
//     //         (None, false)
//     //     };

//     //     RenderMesh {
//     //         vao,
//     //         _vbo: vbo,
//     //         ebo,
//     //         vertex_count: if has_indices {
//     //             prim.indices.as_ref().unwrap().len() as i32
//     //         } else {
//     //             vertex_count as i32
//     //         },
//     //         has_indices,
//     //     }
//     // }
//     fn create_render_mesh(gl: Arc<GL>, prim: Primitive) -> RenderMesh {
//         let vertex_count = prim.positions.len() / 3;

//         // Interleave vertex data: [pos.x, pos.y, pos.z, normal.x, normal.y, normal.z, uv.x, uv.y]
//         let mut vertices = Vec::with_capacity(vertex_count * 8);

//         for i in 0..vertex_count {
//             // Position
//             vertices.push(prim.positions[i * 3]);
//             vertices.push(prim.positions[i * 3 + 1]);
//             vertices.push(prim.positions[i * 3 + 2]);

//             // Normal (or default)
//             if let Some(ref normals) = prim.normals {
//                 vertices.push(normals[i * 3]);
//                 vertices.push(normals[i * 3 + 1]);
//                 vertices.push(normals[i * 3 + 2]);
//             } else {
//                 vertices.push(0.0);
//                 vertices.push(1.0);
//                 vertices.push(0.0);
//             }

//             // TexCoords (or default)
//             if let Some(ref uvs) = prim.tex_coords {
//                 vertices.push(uvs[i * 2]);
//                 vertices.push(uvs[i * 2 + 1]);
//             } else {
//                 vertices.push(0.0);
//                 vertices.push(0.0);
//             }
//         }

//         let vao = VAO::new(gl.clone());
//         vao.bind();

//         let vbo = VBO::new(gl.clone(), &vertices);

//         let stride = 8 * std::mem::size_of::<f32>() as i32;

//         // Position attribute (location = 0)
//         vao.attrib_pointer(
//             0,
//             3,
//             gl.data_type.float,
//             gl.boolean.false_,
//             stride,
//             std::ptr::null(),
//         );

//         // Normal attribute (location = 1)
//         vao.attrib_pointer(
//             1,
//             3,
//             gl.data_type.float,
//             gl.boolean.false_,
//             stride,
//             (3 * std::mem::size_of::<f32>()) as *const ffi::c_void,
//         );

//         // TexCoord attribute (location = 2)
//         vao.attrib_pointer(
//             2,
//             2,
//             gl.data_type.float,
//             gl.boolean.false_,
//             stride,
//             (6 * std::mem::size_of::<f32>()) as *const ffi::c_void,
//         );

//         // Calculate vertex count and handle indices before moving prim.indices
//         let index_count = prim.indices.as_ref().map(|i| i.len() as i32);
//         let (ebo, has_indices) = if let Some(indices) = prim.indices {
//             (Some(EBO::new(gl.clone(), &indices)), true)
//         } else {
//             (None, false)
//         };

//         RenderMesh {
//             vao,
//             _vbo: vbo,
//             ebo,
//             vertex_count: index_count.unwrap_or(vertex_count as i32),
//             has_indices,
//         }
//     }
//     pub fn render(&self, gl: &GL) {
//         for mesh in &self.meshes {
//             mesh.vao.bind();

//             if mesh.has_indices {
//                 if let Some(ref ebo) = mesh.ebo {
//                     ebo.bind();
//                     gl.draw_elements(
//                         gl.primitive.triangles,
//                         mesh.vertex_count,
//                         gl.data_type.unsigned_int,
//                         std::ptr::null(),
//                     );
//                 }
//             } else {
//                 gl.draw_arrays(gl.primitive.triangles, 0, mesh.vertex_count);
//             }
//         }
//     }
// }
use crate::gl::core::*;
use crate::gltf::*;
use crate::math::math::*;
use crate::renderer::{buffer::*, shader::Shader, vertex_array::*};
use std::ffi;
use std::sync::Arc;

pub struct GltfRenderable {
    pub meshes: Vec<RenderMesh>,
    pub model: Mat4,
}

pub struct RenderMesh {
    vao: VAO,
    _vbo: VBO,
    ebo: Option<EBO>,
    vertex_count: i32,
    has_indices: bool,
    material: Material,
}

impl GltfRenderable {
    pub fn from_gltf(gl: Arc<GL>, gltf_model: GltfModel) -> Self {
        let meshes = gltf_model
            .meshes
            .into_iter()
            .flat_map(|mesh| {
                mesh.primitives
                    .into_iter()
                    .map(|prim| Self::create_render_mesh(gl.clone(), prim))
            })
            .collect();

        Self {
            meshes,
            model: Mat4::identity(),
        }
    }

    pub fn load(gl: Arc<GL>, path: &str) -> Result<Self, String> {
        let gltf_model = GltfModel::load(path)?;
        Ok(Self::from_gltf(gl, gltf_model))
    }

    fn create_render_mesh(gl: Arc<GL>, prim: Primitive) -> RenderMesh {
        let vertex_count = prim.positions.len() / 3;
        let material = prim.material.clone();

        let mut vertices = Vec::with_capacity(vertex_count * 8);

        for i in 0..vertex_count {
            vertices.push(prim.positions[i * 3]);
            vertices.push(prim.positions[i * 3 + 1]);
            vertices.push(prim.positions[i * 3 + 2]);

            if let Some(ref normals) = prim.normals {
                vertices.push(normals[i * 3]);
                vertices.push(normals[i * 3 + 1]);
                vertices.push(normals[i * 3 + 2]);
            } else {
                vertices.push(0.0);
                vertices.push(1.0);
                vertices.push(0.0);
            }

            if let Some(ref uvs) = prim.tex_coords {
                vertices.push(uvs[i * 2]);
                vertices.push(uvs[i * 2 + 1]);
            } else {
                vertices.push(0.0);
                vertices.push(0.0);
            }
        }

        let vao = VAO::new(gl.clone());
        vao.bind();

        let vbo = VBO::new(gl.clone(), &vertices);

        let stride = 8 * std::mem::size_of::<f32>() as i32;

        vao.attrib_pointer(
            0,
            3,
            gl.data_type.float,
            gl.boolean.false_,
            stride,
            std::ptr::null(),
        );

        vao.attrib_pointer(
            1,
            3,
            gl.data_type.float,
            gl.boolean.false_,
            stride,
            (3 * std::mem::size_of::<f32>()) as *const ffi::c_void,
        );

        vao.attrib_pointer(
            2,
            2,
            gl.data_type.float,
            gl.boolean.false_,
            stride,
            (6 * std::mem::size_of::<f32>()) as *const ffi::c_void,
        );

        let index_count = prim.indices.as_ref().map(|i| i.len() as i32);
        let (ebo, has_indices) = if let Some(indices) = prim.indices {
            (Some(EBO::new(gl.clone(), &indices)), true)
        } else {
            (None, false)
        };

        RenderMesh {
            vao,
            _vbo: vbo,
            ebo,
            vertex_count: index_count.unwrap_or(vertex_count as i32),
            has_indices,
            material,
        }
    }

    pub fn render(&self, gl: &GL, shader: &Shader) {
        for mesh in &self.meshes {
            mesh.bind_material(shader);

            mesh.vao.bind();

            if mesh.has_indices {
                if let Some(ref ebo) = mesh.ebo {
                    ebo.bind();
                    gl.draw_elements(
                        gl.primitive.triangles,
                        mesh.vertex_count,
                        gl.data_type.unsigned_int,
                        std::ptr::null(),
                    );
                }
            } else {
                gl.draw_arrays(gl.primitive.triangles, 0, mesh.vertex_count);
            }
        }
    }
}

impl RenderMesh {
    fn bind_material(&self, shader: &Shader) {
        shader.set_uniform_4f(
            "baseColorFactor",
            self.material.base_color_factor[0],
            self.material.base_color_factor[1],
            self.material.base_color_factor[2],
            self.material.base_color_factor[3],
        );

        shader.set_uniform_3f(
            "emissiveFactor",
            self.material.emissive_factor[0],
            self.material.emissive_factor[1],
            self.material.emissive_factor[2],
        );

        shader.set_uniform_1f("metallicFactor", self.material.metallic_factor);
        shader.set_uniform_1f("roughnessFactor", self.material.roughness_factor);
        shader.set_uniform_1i("alphaMode", self.material.alpha_mode.as_int());
        shader.set_uniform_1i(
            "doubleSided",
            if self.material.double_sided { 1 } else { 0 },
        );
        shader.set_uniform_1i("useTexture", 0);
        shader.set_uniform_1i("ourTexture", 0);
    }
}
