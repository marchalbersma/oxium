use crate::internal_macros::mat::*;
use crate::Vec4;

/// Passes matrix type info followed by additional arguments to callbacks.
macro_rules! mat2x4 {
    ($call:ident$(($($args:tt)*))?) => {
        $call!(Mat2x4<2, Vec4<f32, 4>> {
            [0] => r0: [r0c0, r0c1, r0c2, r0c3],
            [1] => r1: [r1c0, r1c1, r1c2, r1c3],
        } $($($args)*)?);
    };
}

// Struct
mat2x4!(struct_def);

// Constructors
mat2x4!(new_fn(#[allow(clippy::too_many_arguments)]));
mat2x4!(from_row_fn);

// Indexing
mat2x4!(index_impl);

// Unary operations
mat2x4!(neg_impl);

// Binary matrix operations
mat2x4!(add_impl);
mat2x4!(add_assign_impl);
mat2x4!(sub_impl);
mat2x4!(sub_assign_impl);
mat2x4!(mul_impl);
mat2x4!(mul_assign_impl);

#[cfg(test)]
mod tests {
    use crate::internal_macros::mat::tests::*;
    use crate::{Mat2x4, Vec4};

    /// Passes matrix type info and test data followed by additional arguments to a callback.
    macro_rules! mat2x4_data {
        ($call:ident$(($($args:tt)*))?) => {
            $call!(Mat2x4<2, Vec4<f32, 4>> {
                a {
                    [0] => { [0]:  0.1, [1]: 18.9, [2]:  9.9, [3]:  8.8 },
                    [1] => { [0]: 19.9, [1]: 91.1, [2]:  9.7, [3]: 25.3 },
                },
                b {
                    [0] => { [0]: -1.6, [1]:  3.8, [2]: 36.5, [3]: -1.6 },
                    [1] => { [0]: -4.0, [1]: 28.6, [2]: 14.5, [3]: -6.0 },
                },
                c {
                    [0] => { [0]:  0.2, [1]:  0.1, [2]:  1.9, [3]:  6.8 },
                    [1] => { [0]: -2.7, [1]: -0.3, [2]: -1.9, [3]: -1.4 },
                },
                d {
                    [0] => { [0]: -1.1, [1]: -0.7, [2]: -1.9, [3]: -6.6 },
                    [1] => { [0]: -0.6, [1]: -0.5, [2]: -2.0, [3]: -2.1 },
                },
            } $($($args)*)?);
        };
    }

    // Constructors
    mat2x4_data!(new_test);
    mat2x4_data!(from_rows_test);

    // Indexing
    mat2x4_data!(index_test);
    mat2x4!(index_out_of_bounds_test);

    // Unary operations
    mat2x4_data!(neg_test);

    // Binary vector operations
    mat2x4_data!(add_test);
    mat2x4_data!(add_assign_test);
    mat2x4_data!(sub_test);
    mat2x4_data!(sub_assign_test);
    mat2x4_data!(mul_test);
    mat2x4_data!(mul_assign_test);
}
