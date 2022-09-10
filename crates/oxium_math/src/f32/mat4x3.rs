use crate::internal_macros::mat::*;
use crate::Vec3;

/// Passes matrix type info followed by additional arguments to callbacks.
macro_rules! mat4x3 {
    ($call:ident$(($($args:tt)*))?) => {
        $call!(Mat4x3<4, Vec3<f32, 3>> {
            [0] => r0: [r0c0, r0c1, r0c2],
            [1] => r1: [r1c0, r1c1, r1c2],
            [2] => r2: [r2c0, r2c1, r2c2],
            [3] => r3: [r3c0, r3c1, r3c2],
        } $($($args)*)?);
    };
}

// Struct
mat4x3!(struct_def);

// Constructors
mat4x3!(new_fn(#[allow(clippy::too_many_arguments)]));
mat4x3!(from_row_fn);

// Indexing
mat4x3!(index_impl);

// Unary operations
mat4x3!(neg_impl);

// Binary matrix operations
mat4x3!(add_impl);
mat4x3!(add_assign_impl);
mat4x3!(sub_impl);
mat4x3!(sub_assign_impl);
mat4x3!(mul_impl);
mat4x3!(mul_assign_impl);
mat4x3!(div_impl);
mat4x3!(div_assign_impl);

#[cfg(test)]
mod tests {
    use crate::internal_macros::mat::tests::*;
    use crate::{Mat4x3, Vec3};

    /// Passes matrix type info and test data followed by additional arguments to a callback.
    macro_rules! mat4x3_data {
        ($call:ident$(($($args:tt)*))?) => {
            $call!(Mat4x3<4, Vec3<f32, 3>> {
                a {
                    [0] => { [0]:  1.3, [1]:  2.4, [2]:  7.1 },
                    [1] => { [0]:  7.9, [1]:  5.7, [2]:  2.4 },
                    [2] => { [0]:  4.7, [1]:  4.6, [2]:  0.2 },
                    [3] => { [0]:  5.9, [1]:  6.0, [2]:  9.0 },
                },
                b {
                    [0] => { [0]:  6.1, [1]: -8.9, [2]:  7.0 },
                    [1] => { [0]: -0.1, [1]: 96.4, [2]: -2.2 },
                    [2] => { [0]:  9.0, [1]: -1.4, [2]: 74.4 },
                    [3] => { [0]: -9.5, [1]: 62.1, [2]: -1.1 },
                },
                c {
                    [0] => { [0]: -4.6, [1]: -6.9, [2]: -2.0 },
                    [1] => { [0]:  1.6, [1]:  0.9, [2]:  1.0 },
                    [2] => { [0]:  2.9, [1]:  9.0, [2]:  6.7 },
                    [3] => { [0]: -1.8, [1]: -5.3, [2]: -2.0 },
                },
                d {
                    [0] => { [0]: -2.5, [1]: -0.2, [2]: -9.0 },
                    [1] => { [0]: -7.8, [1]: -7.5, [2]: -0.9 },
                    [2] => { [0]: -5.8, [1]: -9.2, [2]: -8.2 },
                    [3] => { [0]: -2.2, [1]: -8.3, [2]: -9.0 },
                },
            } $($($args)*)?);
        };
    }

    // Constructors
    mat4x3_data!(new_test);
    mat4x3_data!(from_rows_test);

    // Indexing
    mat4x3_data!(index_test);
    mat4x3!(index_out_of_bounds_test);

    // Unary operations
    mat4x3_data!(neg_test);

    // Binary vector operations
    mat4x3_data!(add_test);
    mat4x3_data!(add_assign_test);
    mat4x3_data!(sub_test);
    mat4x3_data!(sub_assign_test);
    mat4x3_data!(mul_test);
    mat4x3_data!(mul_assign_test);
    mat4x3_data!(div_test);
    mat4x3_data!(div_assign_test);
}
