use crate::macros::{
    def_mat_struct, impl_mat_from_rows, impl_mat_index, impl_mat_neg, impl_mat_new,
};
use crate::Vec2;

def_mat_struct!(Mat4x2<f32, 4, 2, Vec2>);

impl_mat_new!(
    #[allow(clippy::too_many_arguments)]
    Mat4x2<f32, Vec2> {
        [m00, m01],
        [m10, m11],
        [m20, m21],
        [m30, m31],
    }
);

impl_mat_from_rows!(Mat4x2<Vec2> { r0, r1, r2, r3 });

impl_mat_index!(Mat4x2<4, Vec2> { 0, 1, 2, 3 });

impl_mat_neg!(Mat4x2 { 0, 1, 2, 3 });

#[cfg(test)]
mod tests {
    use crate::macros::{
        test_mat_from_rows, test_mat_index, test_mat_index_out_of_bounds, test_mat_neg,
        test_mat_new,
    };
    use crate::{Mat4x2, Vec2};

    test_mat_new!(Mat4x2 {
        a {
            0: [x: 1.0, y: 2.0],
            1: [x: 3.0, y: 4.0],
            2: [x: 5.0, y: 6.0],
            3: [x: 7.0, y: 8.0],
        },
        b {
            0: [x: 9.0, y: 0.0],
            1: [x: 0.1, y: 0.2],
            2: [x: 0.3, y: 0.4],
            3: [x: 0.5, y: 0.6],
        },
    });

    test_mat_from_rows!(Mat4x2 {
        a {
            0: Vec2::new(4.9, 5.0),
            1: Vec2::new(5.1, 5.2),
            2: Vec2::new(5.3, 5.4),
            3: Vec2::new(5.5, 5.6),
        },
        b {
            0: Vec2::new(5.7, 5.8),
            1: Vec2::new(5.9, 6.0),
            2: Vec2::new(6.1, 6.2),
            3: Vec2::new(6.3, 6.4),
        },
    });

    test_mat_index!(Mat4x2<Vec2> {
        a {
            0: [0.7, 0.8] = [2.3, 2.4],
            1: [0.9, 1.0] = [2.5, 2.6],
            2: [1.1, 1.2] = [2.7, 2.8],
            3: [1.3, 1.4] = [2.9, 3.0],
        },
        b {
            0: [1.5, 1.6] = [3.1, 3.2],
            1: [1.7, 1.8] = [3.3, 3.4],
            2: [1.9, 2.0] = [3.5, 3.6],
            3: [2.1, 2.2] = [3.7, 3.8],
        },
    });

    test_mat_index_out_of_bounds!(Mat4x2<Vec2> {
        [3.9, 4.0],
        [4.1, 4.2],
        [4.3, 4.4],
        [4.5, 4.6],
    } 4 = [4.7, 4.8]);

    test_mat_neg!(Mat4x2 {
        a {
            6.5, 6.6,
            -6.7, -6.8,
            6.9, 7.0,
            -7.1, -7.2,
        } => {
            -6.5, -6.6,
            6.7, 6.8,
            -6.9, -7.0,
            7.1, 7.2,
        },
        b {
            -7.3, -7.4,
            7.5, -7.6,
            7.7, -7.8,
            -7.9, -8.0,
        } => {
            7.3, 7.4,
            -7.5, 7.6,
            -7.7, 7.8,
            7.9, 8.0,
        },
    });
}
