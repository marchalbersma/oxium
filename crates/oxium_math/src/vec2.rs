use crate::macros::{def_vec_struct, impl_vec_index, impl_vec_new};

def_vec_struct!(Vec2<f32, 2> { x, y });

impl_vec_new!(Vec2<f32> { x, y });

impl_vec_index!(Vec2<f32, 2> { 0 => x, 1 => y });

#[cfg(test)]
mod tests {
    use crate::macros::{test_vec_index, test_vec_index_out_of_bounds, test_vec_new};
    use crate::Vec2;

    test_vec_new!(Vec2 {
        a { x: 1.0, y: 2.0 },
        b { x: 3.0, y: 4.0 },
    });

    test_vec_index!(Vec2 {
        a { 0 => x: 5.0 = 9.0, 1 => y: 6.0 = 0.0 },
        b { 0 => x: 7.0 = 0.1, 1 => y: 8.0 = 0.2 },
    });

    test_vec_index_out_of_bounds!(Vec2 { 0.3, 0.4 } 2 = 0.5);
}
