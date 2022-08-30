use crate::macros::{def_vec_struct, impl_vec_new};

def_vec_struct!(Vec3<f32, 3> { x, y, z });

impl_vec_new!(Vec3<f32> { x, y, z });

#[cfg(test)]
mod tests {
    use crate::macros::test_vec_new;
    use crate::Vec3;

    test_vec_new!(Vec3 {
        a { x: 1.0, y: 2.0, z: 3.0 },
        b { x: 4.0, y: 5.0, z: 6.0 },
    });
}
