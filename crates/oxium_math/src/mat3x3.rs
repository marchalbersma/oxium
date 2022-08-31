use crate::macros::{def_mat_struct, impl_mat_index, impl_mat_new};
use crate::Vec3;

def_mat_struct!(Mat3x3<f32, 3, 3, Vec3>);

impl_mat_new!(
    #[allow(clippy::too_many_arguments)]
    Mat3x3<f32, Vec3> {
        [m00, m01, m02],
        [m10, m11, m12],
        [m20, m21, m22],
    }
);

impl_mat_index!(Mat3x3<3, Vec3> { 0, 1, 2 });

#[cfg(test)]
mod tests {
    use crate::macros::{test_mat_index, test_mat_index_out_of_bounds, test_mat_new};
    use crate::{Mat3x3, Vec3};

    test_mat_new!(Mat3x3 {
        a {
            0: [x: 1.0, y: 2.0, z: 3.0],
            1: [x: 4.0, y: 5.0, z: 6.0],
            2: [x: 7.0, y: 8.0, z: 9.0],
        },
        b {
            0: [x: 0.0, y: 0.1, z: 0.2],
            1: [x: 0.3, y: 0.4, z: 0.5],
            2: [x: 0.6, y: 0.7, z: 0.8],
        },
    });

    test_mat_index!(Mat3x3<Vec3> {
        a {
            0: [0.9, 1.0, 1.1] = [2.7, 2.8, 2.9],
            1: [1.2, 1.3, 1.4] = [3.0, 3.1, 3.2],
            2: [1.5, 1.6, 1.7] = [3.3, 3.4, 3.5],
        },
        b {
            0: [1.8, 1.9, 2.0] = [3.6, 3.7, 3.8],
            1: [2.1, 2.2, 2.3] = [3.9, 4.0, 4.1],
            2: [2.4, 2.5, 2.6] = [4.2, 4.3, 4.4],
        },
    });

    test_mat_index_out_of_bounds!(Mat3x3<Vec3> {
        [4.5, 4.6, 4.7],
        [4.7, 4.9, 5.0],
        [5.1, 5.2, 5.3],
    } 3 = [5.4, 5.5, 5.6]);
}
