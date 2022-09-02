use crate::macros::{def_mat_struct, impl_mat_from_rows, impl_mat_index, impl_mat_new};
use crate::Vec4;

def_mat_struct!(Mat2x4<f32, 2, 4, Vec4>);

impl_mat_new!(
    #[allow(clippy::too_many_arguments)]
    Mat2x4<f32, Vec4> {
        [m00, m01, m02, m03],
        [m10, m11, m12, m13],
    }
);

impl_mat_from_rows!(Mat2x4<Vec4> { r0, r1 });

impl_mat_index!(Mat2x4<2, Vec4> { 0, 1 });

#[cfg(test)]
mod tests {
    use crate::macros::{
        test_mat_from_rows, test_mat_index, test_mat_index_out_of_bounds, test_mat_new,
    };
    use crate::{Mat2x4, Vec4};

    test_mat_new!(Mat2x4 {
        a {
            0: [x: 1.0, y: 2.0, z: 3.0, w: 4.0],
            1: [x: 5.0, y: 6.0, z: 7.0, w: 8.0],
        },
        b {
            0: [x: 9.0, y: 0.0, z: 0.1, w: 0.2],
            1: [x: 0.3, y: 0.4, z: 0.5, w: 0.6],
        },
    });

    test_mat_from_rows!(Mat2x4 {
        a {
            0: Vec4::new(5.1, 5.2, 5.3, 5.4),
            1: Vec4::new(5.5, 5.6, 5.7, 5.8),
        },
        b {
            0: Vec4::new(5.9, 6.0, 6.1, 6.2),
            1: Vec4::new(6.3, 6.4, 6.5, 6.6),
        },
    });

    test_mat_index!(Mat2x4<Vec4> {
        a {
            0: [0.7, 0.8, 0.9, 1.0] = [2.3, 2.4, 2.5, 2.6],
            1: [1.1, 1.2, 1.3, 1.4] = [2.7, 2.8, 2.9, 3.0],
        },
        b {
            0: [1.5, 1.6, 1.7, 1.8] = [3.1, 3.2, 3.3, 3.4],
            1: [1.9, 2.0, 2.1, 2.2] = [3.5, 3.6, 3.7, 3.8],
        },
    });

    test_mat_index_out_of_bounds!(Mat2x4<Vec4> {
        [3.9, 4.0, 4.1, 4.2],
        [4.3, 4.4, 4.5, 4.6],
    } 2 = [4.7, 4.8, 4.9, 5.0]);
}
