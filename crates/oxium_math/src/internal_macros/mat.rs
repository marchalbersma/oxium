/// Defines the matrix struct.
macro_rules! struct_def {
    ($mat:ident<$r:literal, $vec:ident<$t:ty, $c:literal>> {
        $([$row:literal] => $row_name:ident: [$($elem_name:ident),*],)*
    }) => {
        #[doc = concat!(" A ", $r, " x ", $c, " matrix with [`", stringify!($t), "`] elements.")]
        #[derive(Clone, Copy, Debug, Default, PartialEq)]
        pub struct $mat {
            rows: [$vec; $r],
        }
    };
}
pub(crate) use struct_def;

/// Defines the `new()` associated function which creates a matrix from individual elements.
macro_rules! new_fn {
    ($mat:ident<$r:literal, $vec:ident<$t:ty, $c:literal>> {
        $([$row:literal] => $row_name:ident: [$($elem_name:ident),*],)*
    } $(#[$meta:meta])?) => {
        impl $mat {
            /// Creates a new matrix from the given elements.
            $(#[$meta])?
            pub const fn new($($($elem_name: $t),*),*) -> Self {
                Self { rows: [$($vec::new($($elem_name),*)),*] }
            }
        }
    };
}
pub(crate) use new_fn;

/// Defines the `from_rows()` associated function which creates a matrix from row vectors.
macro_rules! from_row_fn {
    ($mat:ident<$r:literal, $vec:ident<$t:ty, $c:literal>> {
        $([$row:literal] => $row_name:ident: [$($elem_name:ident),*],)*
    }) => {
        impl $mat {
            /// Creates a new matrix from the given rows.
            pub const fn from_rows($($row_name: $vec),*) -> Self {
                Self { rows: [$($row_name),*] }
            }
        }
    };
}
pub(crate) use from_row_fn;

/// Implements the [`Index`](std::ops::Index) and [`IndexMut`](std::ops::IndexMut) traits to access matrix rows by 0-based index.
macro_rules! index_impl {
    ($mat:ident<$r:literal, $vec:ident<$t:ty, $c:literal>> {
        $([$row:literal] => $row_name:ident: [$($elem_name:ident),*],)*
    }) => {
        impl std::ops::Index<usize> for $mat {
            /// The row type.
            type Output = $vec;

            /// Accesses matrix rows by 0-based index.
            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    $($row => &self.rows[$row],)*
                    _ => {
                        panic!("index out of bounds: the matrix has {} rows but the index is {}", $r, index)
                    }
                }
            }
        }

        // TODO: remove duplicate code by placing a custom attribute macro on the above impl.
        impl std::ops::IndexMut<usize> for $mat {
            /// Accesses matrix rows by 0-based index.
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                match index {
                    $($row => &mut self.rows[$row],)*
                    _ => {
                        panic!("index out of bounds: the matrix has {} rows but the index is {}", $r, index)
                    }
                }
            }
        }
    };
}
pub(crate) use index_impl;

/// Implements the [`Neg`](std::ops::Neg) trait to perform element-wise matrix negation.
macro_rules! neg_impl {
    ($mat:ident<$r:literal, $vec:ident<$t:ty, $c:literal>> {
        $([$row:literal] => $row_name:ident: [$($elem_name:ident),*],)*
    }) => {
        impl std::ops::Neg for $mat {
            /// The resulting type after negating a matrix.
            type Output = $mat;

            /// Performs element-wise matrix negation.
            fn neg(self) -> Self::Output {
                $mat::from_rows($(-self[$row]),*)
            }
        }

        // TODO: remove duplicate code by placing a custom attribute macro on the above impl.
        impl std::ops::Neg for &$mat {
            /// The resulting type after negating a matrix.
            type Output = $mat;

            /// Performs element-wise matrix negation.
            fn neg(self) -> Self::Output {
                $mat::from_rows($(-self[$row]),*)
            }
        }
    };
}
pub(crate) use neg_impl;

/// Implements the [`Add`](std::ops::Add) trait to perform element-wise matrix addition.
macro_rules! add_impl {
    ($mat:ident<$r:literal, $vec:ident<$t:ty, $c:literal>> {
        $([$row:literal] => $row_name:ident: [$($elem_name:ident),*],)*
    }) => {
        impl std::ops::Add<$mat> for $mat {
            /// The resulting type after adding 2 matrices.
            type Output = $mat;

            /// Performs element-wise matrix addition.
            fn add(self, other: $mat) -> Self::Output {
                $mat::from_rows($(self[$row] + other[$row]),*)
            }
        }

        // TODO: remove duplicate code by placing a custom attribute macro on the first impl.
        impl std::ops::Add<&$mat> for $mat {
            /// The resulting type after adding 2 matrices.
            type Output = Self;

            /// Performs element-wise matrix addition.
            fn add(self, other: &$mat) -> Self::Output {
                $mat::from_rows($(self[$row] + other[$row]),*)
            }
        }

        // TODO: remove duplicate code by placing a custom attribute macro on the first impl.
        impl std::ops::Add<$mat> for &$mat {
            /// The resulting type after adding 2 matrices.
            type Output = $mat;

            /// Performs element-wise matrix addition.
            fn add(self, other: $mat) -> Self::Output {
                $mat::from_rows($(self[$row] + other[$row]),*)
            }
        }

        // TODO: remove duplicate code by placing a custom attribute macro on the first impl.
        impl std::ops::Add<&$mat> for &$mat {
            /// The resulting type after adding 2 matrices.
            type Output = $mat;

            /// Performs element-wise matrix addition.
            fn add(self, other: &$mat) -> Self::Output {
                $mat::from_rows($(self[$row] + other[$row]),*)
            }
        }
    }
}
pub(crate) use add_impl;

/// Implements the [`AddAssign`](std::ops::AddAssign) trait to perform element-wise matrix addition assignment.
macro_rules! add_assign_impl {
    ($mat:ident<$r:literal, $vec:ident<$t:ty, $c:literal>> {
        $([$row:literal] => $row_name:ident: [$($elem_name:ident),*],)*
    }) => {
        impl std::ops::AddAssign<$mat> for $mat {
            /// Performs element-wise matrix addition assignment.
            fn add_assign(&mut self, other: $mat) {
                $(self[$row] += other[$row];)*
            }
        }

        // TODO: remove duplicate code by placing a custom attribute macro on the above impl.
        impl std::ops::AddAssign<&$mat> for $mat {
            /// Performs element-wise matrix addition assignment.
            fn add_assign(&mut self, other: &$mat) {
                $(self[$row] += other[$row];)*
            }
        }
    };
}
pub(crate) use add_assign_impl;

/// Implements the [`Sub`](std::ops::Sub) trait to perform element-wise matrix subtraction.
macro_rules! sub_impl {
    ($mat:ident<$r:literal, $vec:ident<$t:ty, $c:literal>> {
        $([$row:literal] => $row_name:ident: [$($elem_name:ident),*],)*
    }) => {
        impl std::ops::Sub<$mat> for $mat {
            /// The resulting type after subtracting 2 matrices.
            type Output = $mat;

            /// Performs element-wise matrix subtraction.
            fn sub(self, other: $mat) -> Self::Output {
                $mat::from_rows($(self[$row] - other[$row]),*)
            }
        }

        // TODO: remove duplicate code by placing a custom attribute macro on the first impl.
        impl std::ops::Sub<&$mat> for $mat {
            /// The resulting type after subtracting 2 matrices.
            type Output = Self;

            /// Performs element-wise matrix subtraction.
            fn sub(self, other: &$mat) -> Self::Output {
                $mat::from_rows($(self[$row] - other[$row]),*)
            }
        }

        // TODO: remove duplicate code by placing a custom attribute macro on the first impl.
        impl std::ops::Sub<$mat> for &$mat {
            /// The resulting type after subtracting 2 matrices.
            type Output = $mat;

            /// Performs element-wise matrix subtraction.
            fn sub(self, other: $mat) -> Self::Output {
                $mat::from_rows($(self[$row] - other[$row]),*)
            }
        }

        // TODO: remove duplicate code by placing a custom attribute macro on the first impl.
        impl std::ops::Sub<&$mat> for &$mat {
            /// The resulting type after subtracting 2 matrices.
            type Output = $mat;

            /// Performs element-wise matrix subtraction.
            fn sub(self, other: &$mat) -> Self::Output {
                $mat::from_rows($(self[$row] - other[$row]),*)
            }
        }
    }
}
pub(crate) use sub_impl;

/// Implements the [`SubAssign`](std::ops::SubAssign) trait to perform element-wise matrix subtraction assignment.
macro_rules! sub_assign_impl {
    ($mat:ident<$r:literal, $vec:ident<$t:ty, $c:literal>> {
        $([$row:literal] => $row_name:ident: [$($elem_name:ident),*],)*
    }) => {
        impl std::ops::SubAssign<$mat> for $mat {
            /// Performs element-wise matrix subtraction assignment.
            fn sub_assign(&mut self, other: $mat) {
                $(self[$row] -= other[$row];)*
            }
        }

        // TODO: remove duplicate code by placing a custom attribute macro on the above impl.
        impl std::ops::SubAssign<&$mat> for $mat {
            /// Performs element-wise matrix subtraction assignment.
            fn sub_assign(&mut self, other: &$mat) {
                $(self[$row] -= other[$row];)*
            }
        }
    };
}
pub(crate) use sub_assign_impl;

#[cfg(test)]
pub(crate) mod tests {
    /// Creates a test which checks if calling the `new()` associated function correctly sets all matrix elements.
    macro_rules! new_test {
        ($mat:ident<$r:literal, $vec:ident<$t:ty, $c:literal>> { $(
            $var:ident { $(
                [$row:literal] => { $([$col:literal]: $val:literal),* },
            )* },
        )* }) => {
            #[test]
            fn new() { $(
                let $var = $mat::new($($($val,)*)*);
                $($(assert_eq!($var[$row][$col], $val);)*)*
            )* }
        };
    }
    pub(crate) use new_test;

    /// Creates a test which checks if calling the `from_rows()` associated function correctly sets all matrix rows.
    macro_rules! from_rows_test {
        ($mat:ident<$r:literal, $vec:ident<$t:ty, $c:literal>> { $(
            $var:ident { $(
                [$row:literal] => { $([$col:literal]: $val:literal),* },
            )* },
        )* }) => {
            #[test]
            fn from_rows() { $(
                let $var = $mat::from_rows($($vec::new($($val),*)),*);
                $(assert_eq!($var[$row], $vec::new($($val),*));)*
            )* }
        }
    }
    pub(crate) use from_rows_test;

    /// Creates a test which checks if accessing matrix rows by 0-based index accesses the correct rows.
    macro_rules! index_test {
        ($mat:ident<$r:literal, $vec:ident<$t:ty, $c:literal>> { $(
            $a:ident { $(
                [$a_row:literal] => { $([$a_col:literal]: $a_val:literal),* },
            )* },
            $b:ident { $(
                [$b_row:literal] => { $([$b_col:literal]: $b_val:literal),* },
            )* },
        )* }) => {
            #[test]
            fn index() { $(
                let $a = $mat::new($($($a_val),*),*);
                $(assert_eq!($a[$a_row], $vec::new($($a_val),*));)*
            )* }

            #[test]
            fn index_mut() { $(
                let mut $a = $mat::new($($($a_val),*),*);
                $(
                    $a[$a_row] = $vec::new($($b_val),*);
                    assert_eq!($a[$a_row], $vec::new($($b_val),*));
                )*
            )* }
        };
    }
    pub(crate) use index_test;

    /// Creates a test which checks if attempting to access an out of bounds matrix row panics.
    macro_rules! index_out_of_bounds_test {
        ($mat:ident<$r:literal, $vec:ident<$t:ty, $c:literal>> {
            $([$row:literal] => $row_name:ident: [$($elem_name:ident),*],)*
        }) => {
            #[test]
            #[should_panic(expected = "index out of bounds")]
            fn index_out_of_bounds() {
                let mat = $mat::default();
                mat[$r];
            }

            #[test]
            #[should_panic(expected = "index out of bounds")]
            fn index_out_of_bounds_mut() {
                let mut mat = $mat::default();
                mat[$r] = $vec::default();
            }
        };
    }
    pub(crate) use index_out_of_bounds_test;

    /// Creates a test which checks if negating a matrix negates all its elements.
    macro_rules! neg_test {
        ($mat:ident<$r:literal, $vec:ident<$t:ty, $c:literal>> { $(
            $var:ident { $(
                [$row:literal] => { $([$col:literal]: $val:literal),* },
            )* },
        )* }) => {
            #[test]
            #[allow(clippy::double_neg)]
            fn neg() { $(
                let $var = $mat::new($($($val),*),*);
                assert_eq!(-$var, $mat::new($($(-$val),*),*));
                assert_eq!(-&$var, $mat::new($($(-$val),*),*));
            )* }
        };
    }
    pub(crate) use neg_test;

    /// Creates a test which checks if adding 2 matrices adds all their elements.
    macro_rules! add_test {
        ($mat:ident<$r:literal, $vec:ident<$t:ty, $c:literal>> { $(
            $a:ident { $(
                [$a_row:literal] => { $([$a_col:literal]: $a_val:literal),* },
            )* },
            $b:ident { $(
                [$b_row:literal] => { $([$b_col:literal]: $b_val:literal),* },
            )* },
        )* }) => {
            #[test]
            fn add() { $(
                let $a = $mat::new($($($a_val),*),*);
                let $b = $mat::new($($($b_val),*),*);
                assert_eq!( $a +  $b, $mat::new($($($a_val + $b_val),*),*));
                assert_eq!( $a + &$b, $mat::new($($($a_val + $b_val),*),*));
                assert_eq!(&$a +  $b, $mat::new($($($a_val + $b_val),*),*));
                assert_eq!(&$a + &$b, $mat::new($($($a_val + $b_val),*),*));
            )* }
        };
    }
    pub(crate) use add_test;

    /// Creates a test which checks if add-assigning 2 matrices add-assigns all their elements.
    macro_rules! add_assign_test {
        ($mat:ident<$r:literal, $vec:ident<$t:ty, $c:literal>> { $(
            $a:ident { $(
                [$a_row:literal] => { $([$a_col:literal]: $a_val:literal),* },
            )* },
            $b:ident { $(
                [$b_row:literal] => { $([$b_col:literal]: $b_val:literal),* },
            )* },
        )* }) => {
            #[test]
            fn add_assign() { $(
                let mut $a = $mat::new($($($a_val),*),*);
                let $b = $mat::new($($($b_val),*),*);
                $a += $b;
                assert_eq!($a, $mat::new($($($a_val + $b_val),*),*));
                $a += &$b;
                assert_eq!($a, $mat::new($($($a_val + $b_val + $b_val),*),*));
            )* }
        };
    }
    pub(crate) use add_assign_test;

    /// Creates a test which checks if subtracting 2 matrices subtracts all their elements.
    macro_rules! sub_test {
        ($mat:ident<$r:literal, $vec:ident<$t:ty, $c:literal>> { $(
            $a:ident { $(
                [$a_row:literal] => { $([$a_col:literal]: $a_val:literal),* },
            )* },
            $b:ident { $(
                [$b_row:literal] => { $([$b_col:literal]: $b_val:literal),* },
            )* },
        )* }) => {
            #[test]
            fn sub() { $(
                let $a = $mat::new($($($a_val),*),*);
                let $b = $mat::new($($($b_val),*),*);
                assert_eq!( $a -  $b, $mat::new($($($a_val - $b_val),*),*));
                assert_eq!( $a - &$b, $mat::new($($($a_val - $b_val),*),*));
                assert_eq!(&$a -  $b, $mat::new($($($a_val - $b_val),*),*));
                assert_eq!(&$a - &$b, $mat::new($($($a_val - $b_val),*),*));
            )* }
        };
    }
    pub(crate) use sub_test;

    /// Creates a test which checks if subtract-assigning 2 matrices subtract-assigns all their elements.
    macro_rules! sub_assign_test {
        ($mat:ident<$r:literal, $vec:ident<$t:ty, $c:literal>> { $(
            $a:ident { $(
                [$a_row:literal] => { $([$a_col:literal]: $a_val:literal),* },
            )* },
            $b:ident { $(
                [$b_row:literal] => { $([$b_col:literal]: $b_val:literal),* },
            )* },
        )* }) => {
            #[test]
            fn sub_assign() { $(
                let mut $a = $mat::new($($($a_val),*),*);
                let $b = $mat::new($($($b_val),*),*);
                $a -= $b;
                assert_eq!($a, $mat::new($($($a_val - $b_val),*),*));
                $a -= &$b;
                assert_eq!($a, $mat::new($($($a_val - $b_val - $b_val),*),*));
            )* }
        };
    }
    pub(crate) use sub_assign_test;
}
