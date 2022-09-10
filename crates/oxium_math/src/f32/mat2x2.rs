use crate::internal_macros::mat::*;
use crate::Vec2;

/// Passes matrix type info followed by additional arguments to callbacks.
macro_rules! mat2x2 {
    ($call:ident$(($($args:tt)*))?) => {
        $call!(Mat2x2<2, Vec2<f32, 2>> {
            [0] => r0: [r0c0, r0c1],
            [1] => r1: [r1c0, r1c1],
        } $($($args)*)?);
    };
}

// Struct
mat2x2!(struct_def);

// Constructors
mat2x2!(new_fn);
mat2x2!(from_row_fn);

// Indexing
mat2x2!(index_impl);

// Unary operations
mat2x2!(neg_impl);

// Binary matrix operations
mat2x2!(add_impl);
mat2x2!(add_assign_impl);
mat2x2!(sub_impl);
mat2x2!(sub_assign_impl);

#[cfg(test)]
mod tests {
    use crate::internal_macros::mat::tests::*;
    use crate::{Mat2x2, Vec2};

    /// Passes matrix type info and test data followed by additional arguments to a callback.
    macro_rules! mat2x2_data {
        ($call:ident$(($($args:tt)*))?) => {
            $call!(Mat2x2<2, Vec2<f32, 2>> {
                a {
                    [0] => { [0]: 50.5, [1]:  8.4 },
                    [1] => { [0]:  2.5, [1]: 66.2 },
                },
                b {
                    [0] => { [0]:  0.6, [1]: -0.1 },
                    [1] => { [0]:  1.8, [1]: -5.4 },
                },
                c {
                    [0] => { [0]: 91.6, [1]: 22.5 },
                    [1] => { [0]: -5.8, [1]: -8.7 },
                },
                d {
                    [0] => { [0]: -2.1, [1]: -1.0 },
                    [1] => { [0]: -2.0, [1]: -1.5 },
                },
            } $($($args)*)?);
        };
    }

    // Constructors
    mat2x2_data!(new_test);
    mat2x2_data!(from_rows_test);

    // Indexing
    mat2x2_data!(index_test);
    mat2x2!(index_out_of_bounds_test);

    // Unary operations
    mat2x2_data!(neg_test);

    // Binary vector operations
    mat2x2_data!(add_test);
    mat2x2_data!(add_assign_test);
    mat2x2_data!(sub_test);
    mat2x2_data!(sub_assign_test);
}
