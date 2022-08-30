use crate::macros::{def_mat_struct, impl_mat_new};
use crate::Vec4;

def_mat_struct!(Mat2x4<f32, 2, 4, Vec4>);

impl_mat_new!(
    #[allow(clippy::too_many_arguments)]
    Mat2x4<f32, Vec4> {
        [m00, m01, m02, m03],
        [m10, m11, m12, m13],
    }
);

#[cfg(test)]
mod tests {
    use crate::macros::test_mat_new;
    use crate::Mat2x4;

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
}
