use crate::internal_macros::mat::*;
use crate::Vec2;

/// Passes matrix type info followed by additional arguments to callbacks.
macro_rules! mat3x2 {
    ($call:ident$(($($args:tt)*))?) => {
        $call!(Mat3x2<3, Vec2<f32, 2>> {
            [0] => r0: [r0c0, r0c1],
            [1] => r1: [r1c0, r1c1],
            [2] => r2: [r2c0, r2c1],
        } $($($args)*)?);
    };
}

// Struct
mat3x2!(struct_def);

// Constructors
mat3x2!(new_fn);
mat3x2!(from_row_fn);

// Indexing
mat3x2!(index_impl);

// Unary operations
mat3x2!(neg_impl);

// Binary matrix operations
mat3x2!(add_impl);
mat3x2!(add_assign_impl);

#[cfg(test)]
mod tests {
    use crate::internal_macros::mat::tests::*;
    use crate::{Mat3x2, Vec2};

    /// Passes matrix type info and test data followed by additional arguments to a callback.
    macro_rules! mat3x2_data {
        ($call:ident$(($($args:tt)*))?) => {
            $call!(Mat3x2<3, Vec2<f32, 2>> {
                a {
                    [0] => { [0]:  0.0, [1]:  0.1 },
                    [1] => { [0]:  1.3, [1]:  1.3 },
                    [2] => { [0]:  1.6, [1]:  3.0 },
                },
                b {
                    [0] => { [0]:  1.7, [1]: -0.1 },
                    [1] => { [0]:  3.0, [1]: -7.0 },
                    [2] => { [0]:  2.1, [1]: -0.4 },
                },
                c {
                    [0] => { [0]: -1.4, [1]:  3.8 },
                    [1] => { [0]:  1.4, [1]: -7.6 },
                    [2] => { [0]: -1.5, [1]:  3.2 },
                },
                d {
                    [0] => { [0]: -2.9, [1]: -2.2 },
                    [1] => { [0]: -7.7, [1]: -0.2 },
                    [2] => { [0]: -6.5, [1]: -9.6 },
                },
            } $($($args)*)?);
        };
    }

    // Constructors
    mat3x2_data!(new_test);
    mat3x2_data!(from_rows_test);

    // Indexing
    mat3x2_data!(index_test);
    mat3x2!(index_out_of_bounds_test);

    // Unary operations
    mat3x2_data!(neg_test);

    // Binary vector operations
    mat3x2_data!(add_test);
    mat3x2_data!(add_assign_test);
}
