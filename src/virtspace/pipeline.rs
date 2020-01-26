use orbtk::prelude::*;
use euc::{buffer::Buffer2d, rasterizer, Pipeline};
use std::cell::Cell;
use vek::*;

use crate::virtspace::{rgba_to_bgra_u32, objects::*};

#[derive(Clone, Default, PartialEq, Pipeline)]
pub struct VirtSpacePipeline {
    pub frame_counter: Cell<u64>
}

impl render::RenderPipeline for VirtSpacePipeline {
    fn draw(&self, render_target: &mut render::RenderTarget) {
        let mut color = Buffer2d::new(
            [
                render_target.width() as usize,
                render_target.height() as usize,
            ],
            rgba_to_bgra_u32(0, 0, 0, 255),
        );
        let mut depth = Buffer2d::new(
            [
                render_target.width() as usize,
                render_target.height() as usize,
            ],
            1.0,
        );

        let mvp = Mat4::perspective_fov_rh_no(
                1.3, 
                render_target.width() as f32, render_target.height() as f32, 
                0.01, 100.0)
            * Mat4::translation_3d(Vec3::new(-14.0, -3.0, -10.0))
            * Mat4::rotation_x(-0.78);

        // World Grid

        let (world_grid_pos, order, zero_line) = WorldGrid::build((-1, 30), (-1, 15));

        WorldGrid {
            mvp: &mvp,
            positions: &world_grid_pos,
            zero_line_indices: &zero_line
        }
        .draw::<rasterizer::Lines<_>,_>(
            order.as_slice(), &mut color, &mut depth);

        // Rover Body

        let x_pos = (self.frame_counter.get() as f32 * 0.01).sin() * 5.0 + 5.0;
        let y_pos = (self.frame_counter.get() as f32 * 0.01).cos() * 5.0 + 5.0;

        let rov_body = RoverBody::new(
            &mvp,
            Vec3::new(1.0, 1.0, 1.0), 
            Vec3::new(x_pos, y_pos, 0.0), 
            Mat4::rotation_z((self.frame_counter.get() as f32 * 0.01).sin() * 3.14));

        rov_body.draw::<rasterizer::Triangles<_, rasterizer::BackfaceCullingEnabled>, _>(
            &RoverBody::face_colours(),
            &mut color,
            &mut depth
        );

        render_target.draw(color.as_ref());
    }
}