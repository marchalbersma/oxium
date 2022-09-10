use crate::internal_macros::mat::*;
use crate::Vec4;

/// Passes matrix type info followed by additional arguments to callbacks.
macro_rules! mat4x4 {
    ($call:ident$(($($args:tt)*))?) => {
        $call!(Mat4x4<4, Vec4<f32, 4>> {
            [0] => r0: [r0c0, r0c1, r0c2, r0c3],
            [1] => r1: [r1c0, r1c1, r1c2, r1c3],
            [2] => r2: [r2c0, r2c1, r2c2, r2c3],
            [3] => r3: [r3c0, r3c1, r3c2, r3c3],
        } $($($args)*)?);
    };
}

// Struct
mat4x4!(struct_def);

// Constructors
mat4x4!(new_fn(#[allow(clippy::too_many_arguments)]));
mat4x4!(from_row_fn);

// Indexing
mat4x4!(index_impl);

// Unary operations
mat4x4!(neg_impl);

// Binary matrix operations
mat4x4!(add_impl);
mat4x4!(add_assign_impl);
mat4x4!(sub_impl);
mat4x4!(sub_assign_impl);
mat4x4!(mul_impl);
mat4x4!(mul_assign_impl);
mat4x4!(div_impl);
mat4x4!(div_assign_impl);

#[cfg(test)]
mod tests {
    use crate::internal_macros::mat::tests::*;
    use crate::{Mat4x4, Vec4};

    /// Passes matrix type info and test data followed by additional arguments to a callback.
    macro_rules! mat4x4_data {
        ($call:ident$(($($args:tt)*))?) => {
            $call!(Mat4x4<4, Vec4<f32, 4>> {
                a {
                    [0] => { [0]:  3.1, [1]:  4.1, [2]:  5.9, [3]:  2.6 },
                    [1] => { [0]:  5.3, [1]:  5.8, [2]:  9.7, [3]:  9.3 },
                    [2] => { [0]:  2.3, [1]:  8.4, [2]:  6.2, [3]:  6.4 },
                    [3] => { [0]:  3.3, [1]:  8.3, [2]:  2.7, [3]:  9.5 },
                },
                b {
                    [0] => { [0]:  0.4, [1]:  0.1, [2]:  1.7, [3]:  8.5 },
                    [1] => { [0]:  2.0, [1]: -0.9, [2]: -1.8, [3]:  6.3 },
                    [2] => { [0]:  2.4, [1]: -0.2, [2]: -1.7, [3]:  8.6 },
                    [3] => { [0]:  1.6, [1]:  1.2, [2]:  1.8, [3]:  5.9 },
                },
                c {
                    [0] => { [0]: -1.9, [1]: -0.8, [2]: -1.8, [3]: -7.1 },
                    [1] => { [0]: -3.0, [1]:  0.1, [2]:  1.9, [3]: -4.8 },
                    [2] => { [0]: -1.6, [1]:  0.4, [2]:  1.8, [3]: -6.7 },
                    [3] => { [0]: -3.0, [1]: -0.5, [2]: -1.9, [3]: -1.2 },
                },
                d {
                    [0] => { [0]: -2.7, [1]: -1.8, [2]: -2.8, [3]: -1.8 },
                    [1] => { [0]: -2.8, [1]: -4.5, [2]: -9.0, [3]: -4.5 },
                    [2] => { [0]: -2.3, [1]: -5.3, [2]: -6.0, [3]: -2.8 },
                    [3] => { [0]: -7.4, [1]: -7.1, [2]: -3.5, [3]: -2.6 },
                },
            } $($($args)*)?);
        };
    }

    // Constructors
    mat4x4_data!(new_test);
    mat4x4_data!(from_rows_test);

    // Indexing
    mat4x4_data!(index_test);
    mat4x4!(index_out_of_bounds_test);

    // Unary operations
    mat4x4_data!(neg_test);

    // Binary vector operations
    mat4x4_data!(add_test);
    mat4x4_data!(add_assign_test);
    mat4x4_data!(sub_test);
    mat4x4_data!(sub_assign_test);
    mat4x4_data!(mul_test);
    mat4x4_data!(mul_assign_test);
    mat4x4_data!(div_test);
    mat4x4_data!(div_assign_test);
}
