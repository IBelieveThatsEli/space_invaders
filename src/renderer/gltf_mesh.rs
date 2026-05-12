use crate::gl::core::*;
use crate::gltf::*;
use crate::math::math::*;
use crate::renderer::{buffer::*, shader::Shader, vertex_array::*};
use std::arch::x86_64::_mm_setr_ps;
use std::collections::HashMap;
use std::ffi;
use std::sync::Arc;

pub struct GltfRenderable {
    pub meshes: Vec<RenderMesh>,
    pub model: Mat4,
    pub nodes: Vec<Node>,
    pub node_transforms: Vec<NodeTransform>,
    pub animations: Vec<AnimationClip>,
    pub animation_states: HashMap<String, AnimationState>,
}

pub struct RenderMesh {
    vao: VAO,
    _vbo: VBO,
    ebo: Option<EBO>,
    vertex_count: i32,
    has_indices: bool,
    material: Material,
    node_index: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct AnimationState {
    pub clip_index: usize,
    pub time: f32,
    pub playing: bool,
    pub looping: bool,
    pub speed: f32,
}

impl AnimationState {
    pub fn new(clip_index: usize) -> Self {
        Self {
            clip_index,
            time: 0.0,
            playing: false,
            looping: true,
            speed: 1.0,
        }
    }
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
            .collect::<Vec<_>>();

        let animation_states = gltf_model
            .animations
            .iter()
            .enumerate()
            .map(|(i, clip)| (clip.name.clone(), AnimationState::new(i)))
            .collect::<HashMap<_, _>>();

        let node_transforms = gltf_model
            .nodes
            .iter()
            .map(|n| n.transform.clone())
            .collect::<Vec<_>>();

        Self {
            meshes,
            model: Mat4::identity(),
            nodes: gltf_model.nodes,
            node_transforms,
            animations: gltf_model.animations,
            animation_states,
        }
    }

    pub fn load(gl: Arc<GL>, path: &str) -> Result<Self, String> {
        let gltf_model = GltfModel::load(path)?;
        Ok(Self::from_gltf(gl, gltf_model))
    }

    fn create_render_mesh(gl: Arc<GL>, prim: Primitive) -> RenderMesh {
        let vertex_count = prim.positions.len() / 3;
        let material = prim.material.clone();
        let node_index = prim.node_index;

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
            node_index,
        }
    }

    pub fn play_animation(&mut self, name: &str, looping: bool) {
        if let Some(state) = self.animation_states.get_mut(name) {
            state.playing = true;
            state.looping = looping;
            state.time = 0.0;
        }
    }

    pub fn stop_animation(&mut self, name: &str) {
        if let Some(state) = self.animation_states.get_mut(name) {
            state.playing = false;
            state.time = 0.0;
        }
    }

    pub fn set_animation_speed(&mut self, name: &str, speed: f32) {
        if let Some(state) = self.animation_states.get_mut(name) {
            state.speed = speed;
        }
    }
    pub fn update_animations(&mut self, dt: f32) {
        // Reset node transforms to their bind pose / default transforms.
        self.node_transforms = self.nodes.iter().map(|n| n.transform.clone()).collect();

        // Collect owned copies of all currently playing animation states.
        // This avoids holding mutable borrows into self.animation_states
        // while we process the animations.
        let active_states = self
            .animation_states
            .values()
            .filter(|s| s.playing)
            .cloned()
            .collect::<Vec<AnimationState>>();

        for state in active_states {
            // Extract only the data we need from the clip inside a short scope,
            // so the immutable borrow of self.animations ends before we call
            // self.apply_clip(...), which requires &mut self.
            let (clip_index, clip_name, clip_duration) = match self.animations.get(state.clip_index)
            {
                Some(clip) => (state.clip_index, clip.name.clone(), clip.duration),
                None => continue,
            };

            // Advance animation time.
            let mut new_time = state.time + dt * state.speed;

            if clip_duration > 0.0 {
                if state.looping {
                    new_time %= clip_duration;
                } else if new_time > clip_duration {
                    new_time = clip_duration;
                }
            }

            // Update the runtime state.
            if let Some(runtime_state) = self.animation_states.get_mut(&clip_name) {
                runtime_state.time = new_time;
            }

            // Clone the clip so we no longer borrow self.animations.
            let clip = self.animations[clip_index].clone();

            // Safe: no active immutable borrow of self remains.
            self.apply_clip(&clip, new_time);
        }
    }
    // pub fn update_animations(&mut self, dt: f32) {
    //     self.node_transforms = self.nodes.iter().map(|n| n.transform.clone()).collect();

    //     // let active_states = self
    //     // .animation_states
    //     // .values_mut()
    //     // .filter(|s| s.playing)
    //     // .cloned()
    //     // .collect::<Vec<_>>();
    //     let active_states = self
    //         .animation_states
    //         .values_mut()
    //         .filter(|s| s.playing)
    //         .map(|s| s.clone())
    //         .collect::<Vec<AnimationState>>();

    //     for state in active_states {
    //         let clip = match self.animations.get(state.clip_index) {
    //             Some(clip) => clip,
    //             None => continue,
    //         };

    //         let mut new_time = state.time + dt * state.speed;

    //         if clip.duration > 0.0 {
    //             if state.looping {
    //                 new_time %= clip.duration;
    //             } else if new_time > clip.duration {
    //                 new_time = clip.duration;
    //             }
    //         }

    //         if let Some(runtime_state) = self.animation_states.get_mut(&clip.name) {
    //             runtime_state.time = new_time;
    //         }

    //         self.apply_clip(clip, new_time);
    //     }
    // }

    fn apply_clip(&mut self, clip: &AnimationClip, time: f32) {
        for channel in &clip.channels {
            let sampler = match clip.samplers.get(channel.sampler_index) {
                Some(s) => s,
                None => continue,
            };

            let node_transform = match self.node_transforms.get_mut(channel.target_node) {
                Some(t) => t,
                None => continue,
            };

            match (&channel.target_path, &sampler.output) {
                (AnimationTargetPath::Translation, AnimationOutput::Vec3(values)) => {
                    node_transform.translation =
                        sample_vec3(&sampler.input_times, values, time, &sampler.interpolation);
                }
                (AnimationTargetPath::Scale, AnimationOutput::Vec3(values)) => {
                    node_transform.scale =
                        sample_vec3(&sampler.input_times, values, time, &sampler.interpolation);
                }
                (AnimationTargetPath::Rotation, AnimationOutput::Vec4(values)) => {
                    node_transform.rotation =
                        sample_vec4(&sampler.input_times, values, time, &sampler.interpolation);
                }
                _ => {}
            }
        }
    }

    pub fn render(&self, gl: &GL, shader: &Shader) {
        for mesh in &self.meshes {
            mesh.bind_material(shader);

            let model = if let Some(node_index) = mesh.node_index {
                self.model.mul(&node_transform_to_mat4(
                    self.node_transforms
                        .get(node_index)
                        .unwrap_or(&NodeTransform::default()),
                ))
            } else {
                self.model
            };

            shader.set_uniform_mat4fv("model", 1, gl.boolean.false_, model.value_ptr());

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

    pub fn animation_names(&self) -> Vec<String> {
        self.animation_states.keys().cloned().collect()
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

fn sample_vec3(
    times: &[f32],
    values: &[[f32; 3]],
    time: f32,
    interpolation: &Interpolation,
) -> [f32; 3] {
    if times.is_empty() || values.is_empty() {
        return [0.0, 0.0, 0.0];
    }
    if times.len() == 1 || values.len() == 1 {
        return values[0];
    }

    let index = find_keyframe_index(times, time);
    let next = (index + 1).min(values.len() - 1);

    match interpolation {
        Interpolation::Step => values[index],
        Interpolation::Linear => {
            let t0 = times[index];
            let t1 = times[next];
            let alpha = if t1 > t0 {
                (time - t0) / (t1 - t0)
            } else {
                0.0
            };
            lerp3(values[index], values[next], alpha)
        }
    }
}

fn sample_vec4(
    times: &[f32],
    values: &[[f32; 4]],
    time: f32,
    interpolation: &Interpolation,
) -> [f32; 4] {
    if times.is_empty() || values.is_empty() {
        return [0.0, 0.0, 0.0, 1.0];
    }
    if times.len() == 1 || values.len() == 1 {
        return normalize_quat(values[0]);
    }

    let index = find_keyframe_index(times, time);
    let next = (index + 1).min(values.len() - 1);

    match interpolation {
        Interpolation::Step => normalize_quat(values[index]),
        Interpolation::Linear => {
            let t0 = times[index];
            let t1 = times[next];
            let alpha = if t1 > t0 {
                (time - t0) / (t1 - t0)
            } else {
                0.0
            };
            normalize_quat(lerp4(values[index], values[next], alpha))
        }
    }
}

fn find_keyframe_index(times: &[f32], time: f32) -> usize {
    for i in 0..times.len().saturating_sub(1) {
        if time >= times[i] && time <= times[i + 1] {
            return i;
        }
    }
    times.len().saturating_sub(1)
}

// fn lerp3(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
//     [
//         a[0] + (b[0] - a[0]) * t,
//         a[1] + (b[1] - a[1]) * t,
//         a[2] + (b[2] - a[2]) * t,
//     ]
// }

// fn lerp4(a: [f32; 4], b: [f32; 4], t: f32) -> [f32; 4] {
//     [
//         a[0] + (b[0] - a[0]) * t,
//         a[1] + (b[1] - a[1]) * t,
//         a[2] + (b[2] - a[2]) * t,
//         a[3] + (b[3] - a[3]) * t,
//     ]
// }

// fn normalize_quat(q: [f32; 4]) -> [f32; 4] {
//     let len = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3]).sqrt();
//     if len == 0.0 {
//         [0.0, 0.0, 0.0, 1.0]
//     } else {
//         [q[0] / len, q[1] / len, q[2] / len, q[3] / len]
//     }
// }

fn node_transform_to_mat4(transform: &NodeTransform) -> Mat4 {
    let translation = mat4_translation(transform.translation);
    let rotation = quat_to_mat4(transform.rotation);
    let scale = mat4_scale(transform.scale);

    translation.mul(&rotation).mul(&scale)
}

fn mat4_translation(t: Vec3) -> Mat4 {
    let mut mat = Mat4::identity();
    // mat.values[12] = t[0];
    // mat.values[13] = t[1];
    // mat.values[14] = t[2];
    unsafe {
        mat.cols[3] = _mm_setr_ps(t[0], t[1], t[2], 1.0);
    }
    mat
}

fn mat4_scale(s: [f32; 3]) -> Mat4 {
    let mut mat = Mat4::identity();
    // mat.values[0] = s[0];
    // mat.values[5] = s[1];
    // mat.values[10] = s[2];
    unsafe {
        mat.cols[0] = _mm_setr_ps(s[0], 0.0, 0.0, 0.0);
        mat.cols[1] = _mm_setr_ps(0.0, s[1], 0.0, 0.0);
        mat.cols[2] = _mm_setr_ps(0.0, 0.0, s[2], 0.0);
        mat.cols[3] = _mm_setr_ps(0.0, 0.0, 0.0, 1.0);
    }
    mat
}

fn quat_to_mat4(q: [f32; 4]) -> Mat4 {
    let [x, y, z, w] = normalize_quat(q);

    let xx = x * x;
    let yy = y * y;
    let zz = z * z;
    let xy = x * y;
    let xz = x * z;
    let yz = y * z;
    let wx = w * x;
    let wy = w * y;
    let wz = w * z;
    unsafe {
        // Column-major rotation matrix.
        // Each __m128 represents one column.
        Mat4 {
            cols: [
                // Column 0
                _mm_setr_ps(1.0 - 2.0 * (yy + zz), 2.0 * (xy + wz), 2.0 * (xz - wy), 0.0),
                // Column 1
                _mm_setr_ps(2.0 * (xy - wz), 1.0 - 2.0 * (xx + zz), 2.0 * (yz + wx), 0.0),
                // Column 2
                _mm_setr_ps(2.0 * (xz + wy), 2.0 * (yz - wx), 1.0 - 2.0 * (xx + yy), 0.0),
                // Column 3 (translation)
                _mm_setr_ps(0.0, 0.0, 0.0, 1.0),
            ],
        }
    }
    // Mat4 {
    //     values: [
    //         1.0 - 2.0 * (yy + zz),
    //         2.0 * (xy + wz),
    //         2.0 * (xz - wy),
    //         0.0,
    //         2.0 * (xy - wz),
    //         1.0 - 2.0 * (xx + zz),
    //         2.0 * (yz + wx),
    //         0.0,
    //         2.0 * (xz + wy),
    //         2.0 * (yz - wx),
    //         1.0 - 2.0 * (xx + yy),
    //         0.0,
    //         0.0,
    //         0.0,
    //         0.0,
    //         1.0,
    //     ],
    // }
    // de
}
