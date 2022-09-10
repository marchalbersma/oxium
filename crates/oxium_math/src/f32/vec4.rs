use crate::internal_macros::vec::*;

/// Passes vector type info followed by additional arguments to a callback.
macro_rules! vec4 {
    ($call:ident$(($($args:tt)*))?) => {
        $call!(Vec4<f32, 4> {
            [0] => x,
            [1] => y,
            [2] => z,
            [3] => w,
        } $($($args)*)?);
    };
}

// Struct
vec4!(struct_def);

// Constructors
vec4!(new_fn);

// Indexing
vec4!(index_impl);

// Unary operations
vec4!(neg_impl);

// Binary vector operations
vec4!(add_impl);
vec4!(add_assign_impl);
vec4!(sub_impl);
vec4!(sub_assign_impl);

#[cfg(test)]
mod tests {
    use crate::internal_macros::vec::tests::*;
    use crate::Vec4;

    /// Passes vector type info and test data followed by additional arguments to a callback.
    macro_rules! vec4_data {
        ($call:ident$(($($args:tt)*))?) => {
            $call!(Vec4<f32, 4> {
                a { [0] => x:  4.8, [1] => y:  7.5, [2] => z:  0.0, [3] => w:  0.3 },
                b { [0] => x:  0.2, [1] => y: -0.2, [2] => z:  1.9, [3] => w: -9.3 },
                c { [0] => x: -2.8, [1] => y: -0.6, [2] => z:  4.2, [3] => w:  1.2 },
                d { [0] => x: -0.7, [1] => y: -0.9, [2] => z: -1.9, [3] => w: -5.8 },
            } $($($args)*)?);
        };
    }

    // Constructors
    vec4_data!(new_test);

    // Indexing
    vec4_data!(index_test);
    vec4!(index_out_of_bounds_test);

    // Unary operations
    vec4_data!(neg_test);

    // Binary vector operations
    vec4_data!(add_test);
    vec4_data!(add_assign_test);
    vec4_data!(sub_test);
    vec4_data!(sub_assign_test);
}
