use crate::internal_macros::vec::*;

/// Passes vector type info followed by additional arguments to a callback.
macro_rules! vec3 {
    ($call:ident$(($($args:tt)*))?) => {
        $call!(Vec3<f32, 3> {
            [0] => x,
            [1] => y,
            [2] => z,
        } $($($args)*)?);
    };
}

// Struct
vec3!(struct_def);

// Constructors
vec3!(new_fn);

// Indexing
vec3!(index_impl);

// Unary operations
vec3!(neg_impl);

// Binary vector operations
vec3!(add_impl);
vec3!(add_assign_impl);
vec3!(sub_impl);
vec3!(sub_assign_impl);
vec3!(mul_impl);
vec3!(mul_assign_impl);
vec3!(div_impl);
vec3!(div_assign_impl);

#[cfg(test)]
mod tests {
    use crate::internal_macros::vec::tests::*;
    use crate::Vec3;

    /// Passes vector type info and test data followed by additional arguments to a callback.
    macro_rules! vec3_data {
        ($call:ident$(($($args:tt)*))?) => {
            $call!(Vec3<f32, 3> {
                a { [0] => x:  2.3, [1] => y:  8.8, [2] => z:  5.5 },
                b { [0] => x:  3.2, [1] => y: 63.8, [2] => z: -2.7 },
                c { [0] => x: -1.0, [1] => y: -4.8, [2] => z: 59.6 },
                d { [0] => x: -7.6, [1] => y: -3.0, [2] => z: -3.5 },
            } $($($args)*)?);
        };
    }

    // Constructors
    vec3_data!(new_test);

    // Indexing
    vec3_data!(index_test);
    vec3!(index_out_of_bounds_test);

    // Unary operations
    vec3_data!(neg_test);

    // Binary vector operations
    vec3_data!(add_test);
    vec3_data!(add_assign_test);
    vec3_data!(sub_test);
    vec3_data!(sub_assign_test);
    vec3_data!(mul_test);
    vec3_data!(mul_assign_test);
    vec3_data!(div_test);
    vec3_data!(div_assign_test);
}
