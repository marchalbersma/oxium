use crate::internal_macros::mat::*;
use crate::Vec2;

/// Passes matrix type info followed by additional arguments to callbacks.
macro_rules! mat4x2 {
    ($call:ident$(($($args:tt)*))?) => {
        $call!(Mat4x2<4, Vec2<f32, 2>> {
            [0] => r0: [r0c0, r0c1],
            [1] => r1: [r1c0, r1c1],
            [2] => r2: [r2c0, r2c1],
            [3] => r3: [r3c0, r3c1],
        } $($($args)*)?);
    };
}

// Struct
mat4x2!(struct_def);

// Constructors
mat4x2!(new_fn(#[allow(clippy::too_many_arguments)]));
mat4x2!(from_row_fn);

// Indexing
mat4x2!(index_impl);

// Unary operations
mat4x2!(neg_impl);

// Binary matrix operations
mat4x2!(add_impl);
mat4x2!(add_assign_impl);
mat4x2!(sub_impl);
mat4x2!(sub_assign_impl);

#[cfg(test)]
mod tests {
    use crate::internal_macros::mat::tests::*;
    use crate::{Mat4x2, Vec2};

    /// Passes matrix type info and test data followed by additional arguments to a callback.
    macro_rules! mat4x2_data {
        ($call:ident$(($($args:tt)*))?) => {
            $call!(Mat4x2<4, Vec2<f32, 2>> {
                a {
                    [0] => { [0]: 43.2, [1]: 52.0 },
                    [1] => { [0]:  0.3, [1]: 27.4 },
                    [2] => { [0]:  4.8, [1]:  9.8 },
                    [3] => { [0]: 56.0, [1]:  0.0 },
                },
                b {
                    [0] => { [0]:  2.1, [1]: -0.1 },
                    [1] => { [0]:  0.3, [1]: -1.2 },
                    [2] => { [0]:  1.9, [1]: -1.9 },
                    [3] => { [0]:  6.5, [1]: -5.5 },
                },
                c {
                    [0] => { [0]: -1.9, [1]: -1.6 },
                    [1] => { [0]:  1.1, [1]:  1.1 },
                    [2] => { [0]:  1.9, [1]:  2.0 },
                    [3] => { [0]: -9.8, [1]: -0.4 },
                },
                d {
                    [0] => { [0]: -2.0, [1]: -1.5 },
                    [1] => { [0]: -0.8, [1]: -0.3 },
                    [2] => { [0]: -1.8, [1]: -1.9 },
                    [3] => { [0]: -9.0, [1]: -3.7 },
                },
            } $($($args)*)?);
        };
    }

    // Constructors
    mat4x2_data!(new_test);
    mat4x2_data!(from_rows_test);

    // Indexing
    mat4x2_data!(index_test);
    mat4x2!(index_out_of_bounds_test);

    // Unary operations
    mat4x2_data!(neg_test);

    // Binary vector operations
    mat4x2_data!(add_test);
    mat4x2_data!(add_assign_test);
    mat4x2_data!(sub_test);
    mat4x2_data!(sub_assign_test);
}
