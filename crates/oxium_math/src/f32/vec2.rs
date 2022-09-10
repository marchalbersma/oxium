use crate::internal_macros::vec::*;

/// Passes vector type info followed by additional arguments to a callback.
macro_rules! vec2 {
    ($call:ident$(($($args:tt)*))?) => {
        $call!(Vec2<f32, 2> {
            [0] => x,
            [1] => y,
        } $($($args)*)?);
    };
}

// Struct
vec2!(struct_def);

// Constructors
vec2!(new_fn);

// Indexing
vec2!(index_impl);

// Unary operations
vec2!(neg_impl);

// Binary vector operations
vec2!(add_impl);
vec2!(add_assign_impl);
vec2!(sub_impl);
vec2!(sub_assign_impl);

#[cfg(test)]
mod tests {
    use crate::internal_macros::vec::tests::*;
    use crate::Vec2;

    /// Passes vector type info and test data followed by additional arguments to a callback.
    macro_rules! vec2_data {
        ($call:ident$(($($args:tt)*))?) => {
            $call!(Vec2<f32, 2> {
                a { [0] => x:  8.4, [1] => y:  7.2 },
                b { [0] => x:  1.1, [1] => y: -3.8 },
                c { [0] => x: -9.0, [1] => y:  0.1 },
                d { [0] => x: -1.3, [1] => y: -3.7 },
            } $($($args)*)?);
        };
    }

    // Constructors
    vec2_data!(new_test);

    // Indexing
    vec2_data!(index_test);
    vec2!(index_out_of_bounds_test);

    // Unary operations
    vec2_data!(neg_test);

    // Binary vector operations
    vec2_data!(add_test);
    vec2_data!(add_assign_test);
    vec2_data!(sub_test);
    vec2_data!(sub_assign_test);
}
