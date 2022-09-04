use crate::macros::{
    def_vec_struct, impl_vec_add, impl_vec_add_assign, impl_vec_index, impl_vec_neg, impl_vec_new,
};

def_vec_struct!(Vec2<f32, 2> { x, y });

impl_vec_new!(Vec2<f32> { x, y });

impl_vec_index!(Vec2<f32, 2> { 0 => x, 1 => y });

impl_vec_neg!(Vec2 { x, y });

impl_vec_add!(Vec2 { x, y });

impl_vec_add_assign!(Vec2 { x, y });

#[cfg(test)]
mod tests {
    use crate::macros::{
        test_vec_add, test_vec_add_assign, test_vec_index, test_vec_index_out_of_bounds,
        test_vec_neg, test_vec_new,
    };
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

    test_vec_neg!(Vec2 {
        a { 0.6, 0.7 } => { -0.6, -0.7 },
        b { -0.8, -0.9 } => { 0.8, 0.9 },
    });

    test_vec_add!(Vec2 {
        a { 1.0, 1.1 } + b { 1.2, 1.3 } = c,
        d { -1.4, -1.5 } + e { -1.6, -1.7 } = f,
    });

    test_vec_add_assign!(Vec2 {
        a { -1.8, 1.9 } += b { 2.0, -2.1 },
        c { 2.2, -2.3 } += d { -2.4, 2.5 },
    });
}
