pub mod objects;
pub mod shaders;
pub mod pipeline;

use vek::*;

pub fn vec4_array_as_f32_vec(vec4_arr: &[Vec4<f32>]) -> Vec<[f32; 4]> {
    let mut f32_vec: Vec<[f32; 4]> = Vec::with_capacity(vec4_arr.len());

    for i in 0..vec4_arr.len() {
        f32_vec.push([vec4_arr[i].x, vec4_arr[i].y, vec4_arr[i].z, vec4_arr[i].w]);
    }

    f32_vec
}

pub fn rgba_to_bgra_u32(red: u8, green: u8, blue: u8, alpha: u8) -> u32 {
    // Shift together the colors in the order BGRA
    (blue as u32) << 0
        | (green as u32) << 8
        | (red as u32) << 16
        | (alpha as u32) << 24
}