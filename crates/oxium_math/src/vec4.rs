use crate::macros::{def_vec_struct, impl_vec_index, impl_vec_new};

def_vec_struct!(Vec4<f32, 4> { x, y, z, w });

impl_vec_new!(Vec4<f32> { x, y, z, w });

impl_vec_index!(Vec4<f32, 4> { 0 => x, 1 => y, 2 => z, 3 => w });

#[cfg(test)]
mod tests {
    use crate::macros::{test_vec_index, test_vec_index_out_of_bounds, test_vec_new};
    use crate::Vec4;

    test_vec_new!(Vec4 {
        a { x: 1.0, y: 2.0, z: 3.0, w: 4.0 },
        b { x: 5.0, y: 6.0, z: 7.0, w: 8.0 },
    });

    test_vec_index!(Vec4 {
        a { 0 => x: 9.0 = 0.7, 1 => y: 0.0 = 0.8, 2 => z: 0.1 = 0.9, 3 => w: 0.2 = 1.0 },
        b { 0 => x: 0.3 = 1.1, 1 => y: 0.4 = 1.2, 2 => z: 0.5 = 1.3, 3 => w: 0.6 = 1.4 },
    });

    test_vec_index_out_of_bounds!(Vec4 { 1.5, 1.6, 1.7, 1.8 } 4 = 1.9);
}
