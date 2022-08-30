use crate::macros::{def_mat_struct, impl_mat_new};
use crate::Vec4;

def_mat_struct!(Mat4x4<f32, 4, 4, Vec4>);

impl_mat_new!(
    #[allow(clippy::too_many_arguments)]
    Mat4x4<f32, Vec4> {
        [m00, m01, m02, m03],
        [m10, m11, m12, m13],
        [m20, m21, m22, m23],
        [m30, m31, m32, m33],
    }
);

#[cfg(test)]
mod tests {
    use crate::macros::test_mat_new;
    use crate::Mat4x4;

    test_mat_new!(Mat4x4 {
        a {
            0: [x: 1.0, y: 2.0, z: 3.0, w: 4.0],
            1: [x: 5.0, y: 6.0, z: 7.0, w: 8.0],
            2: [x: 9.0, y: 0.0, z: 0.1, w: 0.2],
            3: [x: 0.3, y: 0.4, z: 0.5, w: 0.6],
        },
        b {
            0: [x: 0.7, y: 0.8, z: 0.9, w: 1.0],
            1: [x: 1.1, y: 1.2, z: 1.3, w: 1.4],
            2: [x: 1.5, y: 1.6, z: 1.7, w: 1.8],
            3: [x: 1.9, y: 2.0, z: 2.1, w: 2.2],
        },
    });
}
