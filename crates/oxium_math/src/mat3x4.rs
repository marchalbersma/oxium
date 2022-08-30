use crate::macros::{def_mat_struct, impl_mat_new};
use crate::Vec4;

def_mat_struct!(Mat3x4<f32, 3, 4, Vec4>);

impl_mat_new!(
    #[allow(clippy::too_many_arguments)]
    Mat3x4<f32, Vec4> {
        [m00, m01, m02, m03],
        [m10, m11, m12, m13],
        [m20, m21, m22, m23],
    }
);

#[cfg(test)]
mod tests {
    use crate::macros::test_mat_new;
    use crate::Mat3x4;

    test_mat_new!(Mat3x4 {
        a {
            0: [x: 1.0, y: 2.0, z: 3.0, w: 4.0],
            1: [x: 5.0, y: 6.0, z: 7.0, w: 8.0],
            2: [x: 9.0, y: 0.0, z: 0.1, w: 0.2],
        },
        b {
            0: [x: 0.3, y: 0.4, z: 0.5, w: 0.6],
            1: [x: 0.7, y: 0.8, z: 0.9, w: 1.0],
            2: [x: 1.1, y: 1.2, z: 1.3, w: 1.4],
        },
    });
}
