use crate::macros::{def_mat_struct, impl_mat_new};
use crate::Vec3;

def_mat_struct!(Mat4x3<f32, 4, 3, Vec3>);

impl_mat_new!(
    #[allow(clippy::too_many_arguments)]
    Mat4x3<f32, Vec3> {
        [m00, m01, m02],
        [m10, m11, m12],
        [m20, m21, m22],
        [m30, m31, m32],
    }
);

#[cfg(test)]
mod tests {
    use crate::macros::test_mat_new;
    use crate::Mat4x3;

    test_mat_new!(Mat4x3 {
        a {
            0: [x: 1.0, y: 2.0, z: 3.0],
            1: [x: 4.0, y: 5.0, z: 6.0],
            2: [x: 7.0, y: 8.0, z: 9.0],
            3: [x: 0.0, y: 0.1, z: 0.2],
        },
        b {
            0: [x: 0.3, y: 0.4, z: 0.5],
            1: [x: 0.6, y: 0.7, z: 0.8],
            2: [x: 0.9, y: 1.0, z: 1.1],
            3: [x: 1.2, y: 1.3, z: 1.4],
        },
    });
}
