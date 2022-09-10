use crate::internal_macros::mat::*;
use crate::Vec3;

/// Passes matrix type info followed by additional arguments to callbacks.
macro_rules! mat3x3 {
    ($call:ident$(($($args:tt)*))?) => {
        $call!(Mat3x3<3, Vec3<f32, 3>> {
            [0] => r0: [r0c0, r0c1, r0c2],
            [1] => r1: [r1c0, r1c1, r1c2],
            [2] => r2: [r2c0, r2c1, r2c2],
        } $($($args)*)?);
    };
}

// Struct
mat3x3!(struct_def);

// Constructors
mat3x3!(new_fn(#[allow(clippy::too_many_arguments)]));
mat3x3!(from_row_fn);

// Indexing
mat3x3!(index_impl);

// Unary operations
mat3x3!(neg_impl);

// Binary matrix operations
mat3x3!(add_impl);
mat3x3!(add_assign_impl);

#[cfg(test)]
mod tests {
    use crate::internal_macros::mat::tests::*;
    use crate::{Mat3x3, Vec3};

    /// Passes matrix type info and test data followed by additional arguments to a callback.
    macro_rules! mat3x3_data {
        ($call:ident$(($($args:tt)*))?) => {
            $call!(Mat3x3<3, Vec3<f32, 3>> {
                a {
                    [0] => { [0]:  5.1, [1]:  9.0, [2]: 24.0 },
                    [1] => { [0]:  3.9, [1]: 29.3, [2]:  8.7 },
                    [2] => { [0]: 82.7, [1]:  2.0, [2]:  0.0 },
                },
                b {
                    [0] => { [0]:  9.2, [1]:  2.3, [2]:  3.7 },
                    [1] => { [0]: -2.0, [1]: -4.6, [2]: -8.5 },
                    [2] => { [0]:  4.7, [1]: 55.0, [2]:  9.7 },
                },
                c {
                    [0] => { [0]: -6.6, [1]: 70.9, [2]: -0.3 },
                    [1] => { [0]: 75.2, [1]: -0.2, [2]: 10.7 },
                    [2] => { [0]: -2.9, [1]: 36.9, [2]: -6.0 },
                },
                d {
                    [0] => { [0]: -1.4, [1]: -1.4, [2]: -2.1 },
                    [1] => { [0]: -3.5, [1]: -6.2, [2]: -3.7 },
                    [2] => { [0]: -3.0, [1]: -9.5, [2]: -0.4 },
                },
            } $($($args)*)?);
        };
    }

    // Constructors
    mat3x3_data!(new_test);
    mat3x3_data!(from_rows_test);

    // Indexing
    mat3x3_data!(index_test);
    mat3x3!(index_out_of_bounds_test);

    // Unary operations
    mat3x3_data!(neg_test);

    // Binary vector operations
    mat3x3_data!(add_test);
    mat3x3_data!(add_assign_test);
}
