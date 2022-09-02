use crate::macros::{def_mat_struct, impl_mat_from_rows, impl_mat_index, impl_mat_new};
use crate::Vec2;

def_mat_struct!(Mat2x2<f32, 2, 2, Vec2>);

impl_mat_new!(
    Mat2x2<f32, Vec2> {
        [m00, m01],
        [m10, m11],
    }
);

impl_mat_from_rows!(Mat2x2<Vec2> { r0, r1 });

impl_mat_index!(Mat2x2<2, Vec2> { 0, 1 });

#[cfg(test)]
mod tests {
    use crate::macros::{
        test_mat_from_rows, test_mat_index, test_mat_index_out_of_bounds, test_mat_new,
    };
    use crate::{Mat2x2, Vec2};

    test_mat_new!(Mat2x2 {
        a {
            0: [x: 1.0, y: 2.0],
            1: [x: 3.0, y: 4.0],
        },
        b {
            0: [x: 5.0, y: 6.0],
            1: [x: 7.0, y: 8.0],
        },
    });

    test_mat_from_rows!(Mat2x2 {
        a {
            0: Vec2::new(2.1, 2.2),
            1: Vec2::new(2.3, 2.4),
        },
        b {
            0: Vec2::new(2.5, 2.6),
            1: Vec2::new(2.7, 2.8),
        },
    });

    test_mat_index!(Mat2x2<Vec2> {
        a {
            0: [9.0, 0.0] = [0.7, 0.8],
            1: [0.1, 0.2] = [0.9, 1.0],
        },
        b {
            0: [0.3, 0.4] = [1.1, 1.2],
            1: [0.5, 0.6] = [1.3, 1.4],
        },
    });

    test_mat_index_out_of_bounds!(Mat2x2<Vec2> {
        [1.5, 1.6],
        [1.7, 1.8],
    } 2 = [1.9, 2.0]);
}
