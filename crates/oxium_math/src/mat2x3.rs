use crate::macros::{def_mat_struct, impl_mat_new};
use crate::Vec3;

def_mat_struct!(Mat2x3<f32, 2, 3, Vec3>);

impl_mat_new!(
    Mat2x3<f32, Vec3> {
        [m00, m01, m02],
        [m10, m11, m12],
    }
);

#[cfg(test)]
mod tests {
    use crate::macros::test_mat_new;
    use crate::Mat2x3;

    test_mat_new!(Mat2x3 {
        a {
            0: [x: 1.0, y: 2.0, z: 3.0],
            1: [x: 4.0, y: 5.0, z: 6.0],
        },
        b {
            0: [x: 7.0, y: 8.0, z: 9.0],
            1: [x: 0.0, y: 0.1, z: 0.2],
        },
    });
}
