use crate::macros::{def_mat_struct, impl_mat_new};
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

#[cfg(test)]
mod tests {
    use crate::macros::test_mat_new;
    use crate::Mat4x2;

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
}
