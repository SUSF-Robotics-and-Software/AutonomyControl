use euc::Pipeline;
use std::f32::MAX as F32_MAX;
use vek::*;

use crate::virtspace::{rgba_to_bgra_u32, objects::*};

// ---------------------------------------------------------------------------
// WORLD GRID
// ---------------------------------------------------------------------------

impl<'a> Pipeline for WorldGrid<'a> {
    type Vertex = u32;
    type VsOut = f32;
    type Pixel = u32;

    #[inline(always)]
    fn vert(&self, index: &Self::Vertex) -> ([f32; 4], Self::VsOut) {
        let i = *index as usize;

        let zero_line_index = match self.zero_line_indices.contains(&index) {
            true => F32_MAX,
            false => 0.0
        };

        ((*self.mvp * Vec4::from_point(self.positions[i])).into_array()
            , zero_line_index)
    }

    // Render the triangle in red
    #[inline(always)]
    fn frag(&self, zero_line_index: &Self::VsOut) -> Self::Pixel {
        if *zero_line_index >= F32_MAX - 10.0 {
            rgba_to_bgra_u32(255, 255, 255, 255)
        }
        else {
            rgba_to_bgra_u32(100, 100, 100, 255)
        }
    }
}

// ---------------------------------------------------------------------------
// ROVER BODY
// ---------------------------------------------------------------------------

impl Pipeline for RoverBody {
    type Vertex = (usize, Rgba<f32>);
    type VsOut = Rgba<f32>;
    type Pixel = u32;

    #[inline(always)]
    fn vert(&self, (index, colour): &Self::Vertex) -> ([f32; 4], Self::VsOut) {
        ((self.mvp * self.positions[*index]).into_array(), *colour)
    }

    fn frag(&self, colour: &Self::VsOut) -> Self::Pixel {
        let bytes = colour.map(|e| (e * 255.0) as u8).into_array();
        (bytes[2] as u32) << 0
            | (bytes[1] as u32) << 8
            | (bytes[0] as u32) << 16
            | (bytes[3] as u32) << 24
    }
}