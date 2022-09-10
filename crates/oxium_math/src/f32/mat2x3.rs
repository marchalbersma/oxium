use crate::internal_macros::mat::*;
use crate::Vec3;

/// Passes matrix type info followed by additional arguments to callbacks.
macro_rules! mat2x3 {
    ($call:ident$(($($args:tt)*))?) => {
        $call!(Mat2x3<2, Vec3<f32, 3>> {
            [0] => r0: [r0c0, r0c1, r0c2],
            [1] => r1: [r1c0, r1c1, r1c2],
        } $($($args)*)?);
    };
}

// Struct
mat2x3!(struct_def);

// Constructors
mat2x3!(new_fn);
mat2x3!(from_row_fn);

// Indexing
mat2x3!(index_impl);

// Unary operations
mat2x3!(neg_impl);

// Binary matrix operations
mat2x3!(add_impl);
mat2x3!(add_assign_impl);

#[cfg(test)]
mod tests {
    use crate::internal_macros::mat::tests::*;
    use crate::{Mat2x3, Vec3};

    /// Passes matrix type info and test data followed by additional arguments to a callback.
    macro_rules! mat2x3_data {
        ($call:ident$(($($args:tt)*))?) => {
            $call!(Mat2x3<2, Vec3<f32, 3>> {
                a {
                    [0] => { [0]: 42.0, [1]: 19.8, [2]:  8.6 },
                    [1] => { [0]:  2.3, [1]: 19.0, [2]:  0.5 },
                },
                b {
                    [0] => { [0]: 53.7, [1]: 19.8, [2]:  0.8 },
                    [1] => { [0]:  5.5, [1]: -5.0, [2]: 13.4 },
                },
                c {
                    [0] => { [0]: -2.9, [1]:  9.9, [2]: -8.5 },
                    [1] => { [0]: 59.6, [1]: -7.1, [2]: 34.9 },
                },
                d {
                    [0] => { [0]: -4.2, [1]: -9.7, [2]: -3.3 },
                    [1] => { [0]: -2.2, [1]: -6.4, [2]: -4.4 },
                },
            } $($($args)*)?);
        };
    }

    // Constructors
    mat2x3_data!(new_test);
    mat2x3_data!(from_rows_test);

    // Indexing
    mat2x3_data!(index_test);
    mat2x3!(index_out_of_bounds_test);

    // Unary operations
    mat2x3_data!(neg_test);

    // Binary vector operations
    mat2x3_data!(add_test);
    mat2x3_data!(add_assign_test);
}
