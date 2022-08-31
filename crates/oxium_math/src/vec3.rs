use crate::macros::{def_vec_struct, impl_vec_index, impl_vec_new};

def_vec_struct!(Vec3<f32, 3> { x, y, z });

impl_vec_new!(Vec3<f32> { x, y, z });

impl_vec_index!(Vec3<f32, 3> { 0 => x, 1 => y, 2 => z });

#[cfg(test)]
mod tests {
    use crate::macros::{test_vec_index, test_vec_index_out_of_bounds, test_vec_new};
    use crate::Vec3;

    test_vec_new!(Vec3 {
        a { x: 1.0, y: 2.0, z: 3.0 },
        b { x: 4.0, y: 5.0, z: 6.0 },
    });

    test_vec_index!(Vec3 {
        a { 0 => x: 7.0 = 0.3, 1 => y: 8.0 = 0.4, 2 => z: 9.0 = 0.5 },
        b { 0 => x: 0.0 = 0.6, 1 => y: 0.1 = 0.7, 2 => z: 0.2 = 0.8 },
    });

    test_vec_index_out_of_bounds!(Vec3 { 0.9, 1.0, 1.1 } 3 = 1.2);
}
