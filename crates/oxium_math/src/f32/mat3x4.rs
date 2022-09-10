use crate::internal_macros::mat::*;
use crate::Vec4;

/// Passes matrix type info followed by additional arguments to callbacks.
macro_rules! mat3x4 {
    ($call:ident$(($($args:tt)*))?) => {
        $call!(Mat3x4<3, Vec4<f32, 4>> {
            [0] => r0: [r0c0, r0c1, r0c2, r0c3],
            [1] => r1: [r1c0, r1c1, r1c2, r1c3],
            [2] => r2: [r2c0, r2c1, r2c2, r2c3],
        } $($($args)*)?);
    };
}

// Struct
mat3x4!(struct_def);

// Constructors
mat3x4!(new_fn(#[allow(clippy::too_many_arguments)]));
mat3x4!(from_row_fn);

// Indexing
mat3x4!(index_impl);

// Unary operations
mat3x4!(neg_impl);

// Binary matrix operations
mat3x4!(add_impl);
mat3x4!(add_assign_impl);

#[cfg(test)]
mod tests {
    use crate::internal_macros::mat::tests::*;
    use crate::{Mat3x4, Vec4};

    /// Passes matrix type info and test data followed by additional arguments to a callback.
    macro_rules! mat3x4_data {
        ($call:ident$(($($args:tt)*))?) => {
            $call!(Mat3x4<3, Vec4<f32, 4>> {
                a {
                    [0] => { [0]:  1.6, [1]:  1.8, [2]:  0.3, [3]:  3.9 },
                    [1] => { [0]:  8.8, [1]:  7.4, [2]:  9.8, [3]:  9.4 },
                    [2] => { [0]:  8.4, [1]:  8.2, [2]:  0.4, [3]:  5.8 },
                },
                b {
                    [0] => { [0]:  5.5, [1]: 24.7, [2]:  5.1, [3]: 49.6 },
                    [1] => { [0]: -1.5, [1]: -6.8, [2]: -9.2, [3]: -8.4 },
                    [2] => { [0]: 25.3, [1]:  1.2, [2]: 25.6, [3]:  0.0 },
                },
                c {
                    [0] => { [0]: -2.3, [1]:  0.2, [2]:  5.8, [3]: -5.0 },
                    [1] => { [0]: -9.2, [1]:  9.9, [2]:  4.0, [3]: -4.5 },
                    [2] => { [0]: -6.8, [1]:  4.0, [2]:  1.7, [3]: -9.9 },
                },
                d {
                    [0] => { [0]: -0.3, [1]: -0.7, [2]: -1.9, [3]: -8.5 },
                    [1] => { [0]: -2.2, [1]: -1.1, [2]: -1.9, [3]: -8.9 },
                    [2] => { [0]: -2.5, [1]: -0.5, [2]: -1.9, [3]: -9.0 },
                },
            } $($($args)*)?);
        };
    }

    // Constructors
    mat3x4_data!(new_test);
    mat3x4_data!(from_rows_test);

    // Indexing
    mat3x4_data!(index_test);
    mat3x4!(index_out_of_bounds_test);

    // Unary operations
    mat3x4_data!(neg_test);

    // Binary vector operations
    mat3x4_data!(add_test);
    mat3x4_data!(add_assign_test);
}
