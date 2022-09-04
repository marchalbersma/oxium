use crate::macros::{
    def_mat_struct, impl_mat_add, impl_mat_add_assign, impl_mat_from_rows, impl_mat_index,
    impl_mat_neg, impl_mat_new,
};
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

impl_mat_from_rows!(Mat3x3<Vec3> { r0, r1, r2 });

impl_mat_index!(Mat3x3<3, Vec3> { 0, 1, 2 });

impl_mat_neg!(Mat3x3 { 0, 1, 2 });

impl_mat_add!(Mat3x3 { 0, 1, 2 });

impl_mat_add_assign!(Mat3x3 { 0, 1, 2 });

#[cfg(test)]
mod tests {
    use crate::macros::{
        test_mat_add, test_mat_add_assign, test_mat_from_rows, test_mat_index,
        test_mat_index_out_of_bounds, test_mat_neg, test_mat_new,
    };
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

    test_mat_from_rows!(Mat3x3 {
        a {
            0: Vec3::new(5.7, 5.8, 5.9),
            1: Vec3::new(6.0, 6.1, 6.2),
            2: Vec3::new(6.3, 6.4, 6.5),
        },
        b {
            0: Vec3::new(6.6, 6.7, 6.8),
            1: Vec3::new(6.9, 7.0, 7.1),
            2: Vec3::new(7.2, 7.3, 7.4),
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

    test_mat_neg!(Mat3x3 {
        a {
            7.5, -7.6, 7.7,
            7.8, -7.9, 8.0,
            8.1, -8.2, 8.3,
        } => {
            -7.5, 7.6, -7.7,
            -7.8, 7.9, -8.0,
            -8.1, 8.2, -8.3,
        },
        b {
            -8.4, 8.5, 8.6,
            8.7, -8.8, 8.9,
            9.0, 9.1, -9.2,
        } => {
            8.4, -8.5, -8.6,
            -8.7, 8.8, -8.9,
            -9.0, -9.1, 9.2,
        },
    });

    test_mat_add!(Mat3x3 {
        a {
            9.3, -9.4, 9.5,
            -9.6, 9.7, -9.8,
            9.9, -0.0, 0.1,
        } + b {
            0.2, 0.3, 0.4,
            0.5, 0.6, 0.7,
            0.8, 0.9, 1.0,
        } = c,
        d {
            -1.1, -1.2, -1.3,
            -1.4, -1.5, -1.6,
            -1.7, -1.8, -1.9,
        } + e {
            2.0, -2.1, 2.2,
            2.3, -2.4, 2.5,
            2.6, -2.7, 2.8,
        } = f,
    });

    test_mat_add_assign!(Mat3x3 {
        a {
            -2.9, 3.0, 3.1,
            3.2, -3.3, 3.4,
            3.5, 3.6, -3.7,
        } += b {
            3.8, 3.9, -4.0,
            4.1, -4.2, 4.3,
            -4.4, 4.5, 4.6,
        },
        c {
            -4.7, 4.8, 4.9,
            -5.0, 5.1, 5.2,
            -5.3, 5.4, 5.5,
        } += d {
            5.6, 5.7, 5.8,
            5.9, 6.0, 6.1,
            6.2, 6.3, 6.4,
        },
    });
}
