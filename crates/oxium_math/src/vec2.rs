use crate::macros::{def_vec_struct, impl_vec_new};

def_vec_struct!(Vec2<f32, 2> { x, y });

impl_vec_new!(Vec2<f32> { x, y });

#[cfg(test)]
mod tests {
    use crate::macros::test_vec_new;
    use crate::Vec2;

    test_vec_new!(Vec2 {
        a { x: 1.0, y: 2.0 },
        b { x: 3.0, y: 4.0 },
    });
}
