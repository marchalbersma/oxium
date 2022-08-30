use crate::macros::{def_mat_struct, impl_mat_new};
use crate::Vec2;

def_mat_struct!(Mat2x2<f32, 2, 2, Vec2>);

impl_mat_new!(
    Mat2x2<f32, Vec2> {
        [m00, m01],
        [m10, m11],
    }
);

#[cfg(test)]
mod tests {
    use crate::macros::test_mat_new;
    use crate::Mat2x2;

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
}
