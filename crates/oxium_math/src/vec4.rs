use crate::macros::{def_vec_struct, impl_vec_new};

def_vec_struct!(Vec4<f32, 4> { x, y, z, w });

impl_vec_new!(Vec4<f32> { x, y, z, w });

#[cfg(test)]
mod tests {
    use crate::macros::test_vec_new;
    use crate::Vec4;

    test_vec_new!(Vec4 {
        a { x: 1.0, y: 2.0, z: 3.0, w: 4.0 },
        b { x: 5.0, y: 6.0, z: 7.0, w: 8.0 },
    });
}
