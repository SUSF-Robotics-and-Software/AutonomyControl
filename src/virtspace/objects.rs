use vek::*;

// ---------------------------------------------------------------------------
// WORLD GRID
// ---------------------------------------------------------------------------

pub struct WorldGrid<'a> {
    pub mvp: &'a Mat4<f32>,
    pub positions: &'a [Vec4<f32>],
    pub zero_line_indices: &'a Vec<u32>,
}

impl<'a> WorldGrid<'a> {

    pub fn build(x_range: (i32, i32), y_range: (i32, i32)) -> (Vec<Vec4<f32>>, Vec<u32>, Vec<u32>) {

        let width_x = x_range.1 - x_range.0 + 1;
        let width_y = y_range.1 - y_range.0 + 1;

        let zero_x = (x_range.0..x_range.1).position(|x| x == 0);
        let zero_y = (y_range.0..y_range.1).position(|x| x == 0);

        let num_points = width_x * width_y;
        let mut pos = Vec::with_capacity(num_points as usize);
        let mut draw_order: Vec<u32> = vec![];
        let mut zero_line: Vec<u32> = vec![];

        for i in 0..width_y {
            for j in 0..width_x {
                pos.push(Vec4::new(
                    (x_range.0 + j) as f32, (y_range.0 + i) as f32, 0.0, 1.0));
                
                if j < width_x - 1 {
                    // Right line
                    draw_order.push((i * width_x + j) as u32);
                    draw_order.push((i * width_x + j + 1) as u32);
                }

                if i < width_y - 1 {
                    // Left line
                    draw_order.push((i * width_x + j) as u32);
                    draw_order.push(((i + 1) * width_x + j) as u32);
                }

                // x zero line
                if let Some(z) = zero_x {
                    if z == j as usize {
                        zero_line.push((i * width_x + j) as u32);
                    }
                }

                // y zero line
                if let Some(z) = zero_y {
                    if z == i as usize {
                        zero_line.push((i * width_x + j) as u32);
                    }
                }
            }
        }

        (pos, draw_order, zero_line)
    }
}

// ---------------------------------------------------------------------------
// ROVER BODY
// ---------------------------------------------------------------------------

pub struct RoverBody {
    pub mvp: Mat4<f32>,
    pub positions: Vec<Vec4<f32>>
}

impl RoverBody {

    pub fn new(mvp: &Mat4<f32>, ext: Vec3<f32>, pos: Vec3<f32>, att: Mat4<f32>) -> Self {
        let mvp_new = (*mvp) * Mat4::translation_3d(pos) * att;

        let (l, w, h) = (ext.x * 0.5, ext.y * 0.5, ext.z * 0.5);

        let v_pos = vec![
            Vec4::new(-1.0 * l, -1.0 * w, -1.0 * h, 1.0),  // 0
            Vec4::new(-1.0 * l, -1.0 * w,  1.0 * h, 1.0),  // 1
            Vec4::new(-1.0 * l,  1.0 * w, -1.0 * h, 1.0),  // 2
            Vec4::new(-1.0 * l,  1.0 * w,  1.0 * h, 1.0),  // 3
            Vec4::new( 1.0 * l, -1.0 * w, -1.0 * h, 1.0),  // 4
            Vec4::new( 1.0 * l, -1.0 * w,  1.0 * h, 1.0),  // 5
            Vec4::new( 1.0 * l,  1.0 * w, -1.0 * h, 1.0),  // 6
            Vec4::new( 1.0 * l,  1.0 * w,  1.0 * h, 1.0),  // 7
        ];

        RoverBody {
            mvp: mvp_new,
            positions: v_pos
        }
    }

    pub fn face_colours() -> Vec<(usize, Rgba<f32>)> {
        vec![
            // -x
            (0, Rgba::new(0.66, 0.66, 0.66, 0.0)),
            (3, Rgba::new(0.66, 0.66, 0.66, 0.0)),
            (2, Rgba::new(0.66, 0.66, 0.66, 0.0)),
            (0, Rgba::new(0.66, 0.66, 0.66, 0.0)),
            (1, Rgba::new(0.66, 0.66, 0.66, 0.0)),
            (3, Rgba::new(0.66, 0.66, 0.66, 0.0)),
            // +x
            (7, Rgba::red()),
            (4, Rgba::red()),
            (6, Rgba::red()),
            (5, Rgba::red()),
            (4, Rgba::red()),
            (7, Rgba::red()),
            // -y
            (5, Rgba::new(0.66, 0.66, 0.66, 0.0)),
            (0, Rgba::new(0.66, 0.66, 0.66, 0.0)),
            (4, Rgba::new(0.66, 0.66, 0.66, 0.0)),
            (1, Rgba::new(0.66, 0.66, 0.66, 0.0)),
            (0, Rgba::new(0.66, 0.66, 0.66, 0.0)),
            (5, Rgba::new(0.66, 0.66, 0.66, 0.0)),
            // +y
            (2, Rgba::new(0.66, 0.66, 0.66, 0.0)),
            (7, Rgba::new(0.66, 0.66, 0.66, 0.0)),
            (6, Rgba::new(0.66, 0.66, 0.66, 0.0)),
            (2, Rgba::new(0.66, 0.66, 0.66, 0.0)),
            (3, Rgba::new(0.66, 0.66, 0.66, 0.0)),
            (7, Rgba::new(0.66, 0.66, 0.66, 0.0)),
            // -z
            (0, Rgba::new(0.66, 0.66, 0.66, 0.0)),
            (6, Rgba::new(0.66, 0.66, 0.66, 0.0)),
            (4, Rgba::new(0.66, 0.66, 0.66, 0.0)),
            (0, Rgba::new(0.66, 0.66, 0.66, 0.0)),
            (2, Rgba::new(0.66, 0.66, 0.66, 0.0)),
            (6, Rgba::new(0.66, 0.66, 0.66, 0.0)),
            // +z
            (7, Rgba::new(0.66, 0.66, 0.66, 0.0)),
            (1, Rgba::new(0.66, 0.66, 0.66, 0.0)),
            (5, Rgba::new(0.66, 0.66, 0.66, 0.0)),
            (3, Rgba::new(0.66, 0.66, 0.66, 0.0)),
            (1, Rgba::new(0.66, 0.66, 0.66, 0.0)),
            (7, Rgba::new(0.66, 0.66, 0.66, 0.0))
        ]
    }
}
