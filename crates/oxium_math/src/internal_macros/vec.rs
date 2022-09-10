/// Defines the vector struct.
macro_rules! struct_def {
    ($vec:ident<$t:ty, $n:literal> {
        $([$index:literal] => $comp:ident,)*
    }) => {
        #[doc = concat!(" A ", $n, "-dimensional vector with [`", stringify!($t), "`] components.")]
        #[derive(Clone, Copy, Debug, Default, PartialEq)]
        pub struct $vec { $(
            #[doc = concat!(" The ", stringify!($comp), "-component of the vector.")]
            pub $comp: $t,
        )* }
    };
}
pub(crate) use struct_def;

/// Defines the `new()` associated function which creates a vector from individual components.
macro_rules! new_fn {
    ($vec:ident<$t:ty, $n:literal> {
        $([$index:literal] => $comp:ident,)*
    }) => {
        impl $vec {
            /// Creates a new vector from the given components.
            pub const fn new($($comp: $t),*) -> Self {
                Self { $($comp),* }
            }
        }
    };
}
pub(crate) use new_fn;

/// Implements the [`Index`](std::ops::Index) and [`IndexMut`](std::ops::IndexMut) traits to access vector components by 0-based index.
macro_rules! index_impl {
    ($vec:ident<$t:ty, $n:literal> {
        $([$index:literal] => $comp:ident,)*
    }) => {
        impl std::ops::Index<usize> for $vec {
            /// The component type.
            type Output = $t;

            /// Accesses vector components by 0-based index.
            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    $($index => &self.$comp,)*
                    _ => {
                        panic!("index out of bounds: the vector is {}-dimensional but the index is {}", $n, index)
                    }
                }
            }
        }

        // TODO: remove duplicate code by placing a custom attribute macro on the above impl.
        impl std::ops::IndexMut<usize> for $vec {
            /// Accesses vector components by 0-based index.
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                match index {
                    $($index => &mut self.$comp,)*
                    _ => {
                        panic!("index out of bounds: the vector is {}-dimensional but the index is {}", $n, index)
                    }
                }
            }
        }
    };
}
pub(crate) use index_impl;

/// Implements the [`Neg`](std::ops::Neg) trait to perform component-wise vector negation.
macro_rules! neg_impl {
    ($vec:ident<$t:ty, $n:literal> {
        $([$index:literal] => $comp:ident,)*
    }) => {
        impl std::ops::Neg for $vec {
            /// The resulting type after negating a vector.
            type Output = $vec;

            /// Performs component-wise vector negation.
            fn neg(self) -> Self::Output {
                $vec::new($(-self.$comp),*)
            }
        }

        // TODO: remove duplicate code by placing a custom attribute macro on the above impl.
        impl std::ops::Neg for &$vec {
            /// The resulting type after negating a vector.
            type Output = $vec;

            /// Performs component-wise vector negation.
            fn neg(self) -> Self::Output {
                $vec::new($(-self.$comp),*)
            }
        }
    };
}
pub(crate) use neg_impl;

/// Implements the [`Add`](std::ops::Add) trait to perform component-wise vector addition.
macro_rules! add_impl {
    ($vec:ident<$t:ty, $n:literal> {
        $([$index:literal] => $comp:ident,)*
    }) => {
        impl std::ops::Add<$vec> for $vec {
            /// The resulting type after adding 2 vectors.
            type Output = $vec;

            /// Performs component-wise vector addition.
            fn add(self, other: $vec) -> Self::Output {
                $vec::new($(self.$comp + other.$comp),*)
            }
        }

        // TODO: remove duplicate code by placing a custom attribute macro on the first impl.
        impl std::ops::Add<&$vec> for $vec {
            /// The resulting type after adding 2 vectors.
            type Output = Self;

            /// Performs component-wise vector addition.
            fn add(self, other: &$vec) -> Self::Output {
                $vec::new($(self.$comp + other.$comp),*)
            }
        }

        // TODO: remove duplicate code by placing a custom attribute macro on the first impl.
        impl std::ops::Add<$vec> for &$vec {
            /// The resulting type after adding 2 vectors.
            type Output = $vec;

            /// Performs component-wise vector addition.
            fn add(self, other: $vec) -> Self::Output {
                $vec::new($(self.$comp + other.$comp),*)
            }
        }

        // TODO: remove duplicate code by placing a custom attribute macro on the first impl.
        impl std::ops::Add<&$vec> for &$vec {
            /// The resulting type after adding 2 vectors.
            type Output = $vec;

            /// Performs component-wise vector addition.
            fn add(self, other: &$vec) -> Self::Output {
                $vec::new($(self.$comp + other.$comp),*)
            }
        }
    }
}
pub(crate) use add_impl;

/// Implements the [`AddAssign`](std::ops::AddAssign) trait to perform component-wise vector addition assignment.
macro_rules! add_assign_impl {
    ($vec:ident<$t:ty, $n:literal> {
        $([$index:literal] => $comp:ident,)*
    }) => {
        impl std::ops::AddAssign<$vec> for $vec {
            /// Performs component-wise vector addition assignment.
            fn add_assign(&mut self, other: $vec) {
                $(self.$comp += other.$comp;)*
            }
        }

        // TODO: remove duplicate code by placing a custom attribute macro on the above impl.
        impl std::ops::AddAssign<&$vec> for $vec {
            /// Performs component-wise vector addition assignment.
            fn add_assign(&mut self, other: &$vec) {
                $(self.$comp += other.$comp;)*
            }
        }
    };
}
pub(crate) use add_assign_impl;

/// Implements the [`Sub`](std::ops::Sub) trait to perform component-wise vector subtraction.
macro_rules! sub_impl {
    ($vec:ident<$t:ty, $n:literal> {
        $([$index:literal] => $comp:ident,)*
    }) => {
        impl std::ops::Sub<$vec> for $vec {
            /// The resulting type after subtracting 2 vectors.
            type Output = $vec;

            /// Performs component-wise vector subtraction.
            fn sub(self, other: $vec) -> Self::Output {
                $vec::new($(self.$comp - other.$comp),*)
            }
        }

        // TODO: remove duplicate code by placing a custom attribute macro on the first impl.
        impl std::ops::Sub<&$vec> for $vec {
            /// The resulting type after subtracting 2 vectors.
            type Output = Self;

            /// Performs component-wise vector subtraction.
            fn sub(self, other: &$vec) -> Self::Output {
                $vec::new($(self.$comp - other.$comp),*)
            }
        }

        // TODO: remove duplicate code by placing a custom attribute macro on the first impl.
        impl std::ops::Sub<$vec> for &$vec {
            /// The resulting type after subtracting 2 vectors.
            type Output = $vec;

            /// Performs component-wise vector subtraction.
            fn sub(self, other: $vec) -> Self::Output {
                $vec::new($(self.$comp - other.$comp),*)
            }
        }

        // TODO: remove duplicate code by placing a custom attribute macro on the first impl.
        impl std::ops::Sub<&$vec> for &$vec {
            /// The resulting type after subtracting 2 vectors.
            type Output = $vec;

            /// Performs component-wise vector subtraction.
            fn sub(self, other: &$vec) -> Self::Output {
                $vec::new($(self.$comp - other.$comp),*)
            }
        }
    }
}
pub(crate) use sub_impl;

/// Implements the [`SubAssign`](std::ops::SubAssign) trait to perform component-wise vector subtraction assignment.
macro_rules! sub_assign_impl {
    ($vec:ident<$t:ty, $n:literal> {
        $([$index:literal] => $comp:ident,)*
    }) => {
        impl std::ops::SubAssign<$vec> for $vec {
            /// Performs component-wise vector subtraction assignment.
            fn sub_assign(&mut self, other: $vec) {
                $(self.$comp -= other.$comp;)*
            }
        }

        // TODO: remove duplicate code by placing a custom attribute macro on the above impl.
        impl std::ops::SubAssign<&$vec> for $vec {
            /// Performs component-wise vector subtraction assignment.
            fn sub_assign(&mut self, other: &$vec) {
                $(self.$comp -= other.$comp;)*
            }
        }
    };
}
pub(crate) use sub_assign_impl;

/// Implements the [`Mul`](std::ops::Mul) trait to perform component-wise vector multiplication.
macro_rules! mul_impl {
    ($vec:ident<$t:ty, $n:literal> {
        $([$index:literal] => $comp:ident,)*
    }) => {
        impl std::ops::Mul<$vec> for $vec {
            /// The resulting type after multiplying 2 vectors.
            type Output = $vec;

            /// Performs component-wise vector multiplication.
            fn mul(self, other: $vec) -> Self::Output {
                $vec::new($(self.$comp * other.$comp),*)
            }
        }

        // TODO: remove duplicate code by placing a custom attribute macro on the first impl.
        impl std::ops::Mul<&$vec> for $vec {
            /// The resulting type after multiplying 2 vectors.
            type Output = Self;

            /// Performs component-wise vector multiplication.
            fn mul(self, other: &$vec) -> Self::Output {
                $vec::new($(self.$comp * other.$comp),*)
            }
        }

        // TODO: remove duplicate code by placing a custom attribute macro on the first impl.
        impl std::ops::Mul<$vec> for &$vec {
            /// The resulting type after multiplying 2 vectors.
            type Output = $vec;

            /// Performs component-wise vector multiplication.
            fn mul(self, other: $vec) -> Self::Output {
                $vec::new($(self.$comp * other.$comp),*)
            }
        }

        // TODO: remove duplicate code by placing a custom attribute macro on the first impl.
        impl std::ops::Mul<&$vec> for &$vec {
            /// The resulting type after multiplying 2 vectors.
            type Output = $vec;

            /// Performs component-wise vector multiplication.
            fn mul(self, other: &$vec) -> Self::Output {
                $vec::new($(self.$comp * other.$comp),*)
            }
        }
    }
}
pub(crate) use mul_impl;

/// Implements the [`MulAssign`](std::ops::MulAssign) trait to perform component-wise vector multiplication assignment.
macro_rules! mul_assign_impl {
    ($vec:ident<$t:ty, $n:literal> {
        $([$index:literal] => $comp:ident,)*
    }) => {
        impl std::ops::MulAssign<$vec> for $vec {
            /// Performs component-wise vector multiplication assignment.
            fn mul_assign(&mut self, other: $vec) {
                $(self.$comp *= other.$comp;)*
            }
        }

        // TODO: remove duplicate code by placing a custom attribute macro on the above impl.
        impl std::ops::MulAssign<&$vec> for $vec {
            /// Performs component-wise vector multiplication assignment.
            fn mul_assign(&mut self, other: &$vec) {
                $(self.$comp *= other.$comp;)*
            }
        }
    };
}
pub(crate) use mul_assign_impl;

#[cfg(test)]
pub(crate) mod tests {
    /// Creates a test which checks if calling the `new()` associated function correctly sets all vector components.
    macro_rules! new_test {
        ($vec:ident<$t:ty, $n:literal> { $(
            $var:ident { $([$index:literal] => $comp:ident: $val:literal),* },
        )* }) => {
            #[test]
            fn new() { $(
                let $var = $vec::new($($val),*);
                $(assert_eq!($var.$comp, $val);)*
            )* }
        };
    }
    pub(crate) use new_test;

    /// Creates a test which checks if accessing vector components by 0-based index accesses the correct components.
    macro_rules! index_test {
        ($vec:ident<$t:ty, $n:literal> { $(
            $a:ident { $([$a_index:literal] => $a_comp:ident: $a_val:literal),* },
            $b:ident { $([$b_index:literal] => $b_comp:ident: $b_val:literal),* },
        )* }) => {
            #[test]
            fn index() { $(
                let $a = $vec::new($($a_val),*);
                $(assert_eq!($a[$a_index], $a.$a_comp);)*
            )? }

            #[test]
            fn index_mut() { $(
                let mut $a = $vec::new($($a_val),*);
                $(
                    $a[$a_index] = $b_val;
                    assert_eq!($a[$a_index], $b_val);
                )*
            )? }
        };
    }
    pub(crate) use index_test;

    /// Creates a test which checks if attempting to access an out of bounds vector component panics.
    macro_rules! index_out_of_bounds_test {
        ($vec:ident<$t:ty, $n:literal> {
            $([$index:literal] => $comp:ident,)*
        }) => {
            #[test]
            #[should_panic(expected = "index out of bounds")]
            fn index_out_of_bounds() {
                let vec = $vec::default();
                vec[$n];
            }

            #[test]
            #[should_panic(expected = "index out of bounds")]
            fn index_out_of_bounds_mut() {
                let mut vec = $vec::default();
                vec[$n] = <$t>::default();
            }
        };
    }
    pub(crate) use index_out_of_bounds_test;

    /// Creates a test which checks if negating a vector negates all its components.
    macro_rules! neg_test {
        ($vec:ident<$t:ty, $n:literal> { $(
            $var:ident { $([$index:literal] => $comp:ident: $val:literal),* },
        )* }) => {
            #[test]
            #[allow(clippy::double_neg)]
            fn neg() { $(
                let $var = $vec::new($($val),*);
                assert_eq!(-$var, $vec::new($(-$val),*));
                assert_eq!(-&$var, $vec::new($(-$val),*));
            )* }
        };
    }
    pub(crate) use neg_test;

    /// Creates a test which checks if adding 2 vectors adds all their components.
    macro_rules! add_test {
        ($vec:ident<$t:ty, $n:literal> { $(
            $a:ident { $([$a_index:literal] => $a_comp:ident: $a_val:literal),* },
            $b:ident { $([$b_index:literal] => $b_comp:ident: $b_val:literal),* },
        )* }) => {
            #[test]
            fn add() { $(
                let $a = $vec::new($($a_val),*);
                let $b = $vec::new($($b_val),*);
                assert_eq!( $a +  $b, $vec::new($($a_val + $b_val),*));
                assert_eq!( $a + &$b, $vec::new($($a_val + $b_val),*));
                assert_eq!(&$a +  $b, $vec::new($($a_val + $b_val),*));
                assert_eq!(&$a + &$b, $vec::new($($a_val + $b_val),*));
            )* }
        };
    }
    pub(crate) use add_test;

    /// Creates a test which checks if add-assigning 2 vectors add-assigns all their components.
    macro_rules! add_assign_test {
        ($vec:ident<$t:ty, $n:literal> { $(
            $a:ident { $([$a_index:literal] => $a_comp:ident: $a_val:literal),* },
            $b:ident { $([$b_index:literal] => $b_comp:ident: $b_val:literal),* },
        )* }) => {
            #[test]
            fn add_assign() { $(
                let mut $a = $vec::new($($a_val),*);
                let $b = $vec::new($($b_val),*);
                $a += $b;
                assert_eq!($a, $vec::new($($a_val + $b_val),*));
                $a += &$b;
                assert_eq!($a, $vec::new($($a_val + $b_val + $b_val),*));
            )* }
        };
    }
    pub(crate) use add_assign_test;

    /// Creates a test which checks if subtracting 2 vectors subtracts all their components.
    macro_rules! sub_test {
        ($vec:ident<$t:ty, $n:literal> { $(
            $a:ident { $([$a_index:literal] => $a_comp:ident: $a_val:literal),* },
            $b:ident { $([$b_index:literal] => $b_comp:ident: $b_val:literal),* },
        )* }) => {
            #[test]
            fn sub() { $(
                let $a = $vec::new($($a_val),*);
                let $b = $vec::new($($b_val),*);
                assert_eq!( $a -  $b, $vec::new($($a_val - $b_val),*));
                assert_eq!( $a - &$b, $vec::new($($a_val - $b_val),*));
                assert_eq!(&$a -  $b, $vec::new($($a_val - $b_val),*));
                assert_eq!(&$a - &$b, $vec::new($($a_val - $b_val),*));
            )* }
        };
    }
    pub(crate) use sub_test;

    /// Creates a test which checks if subtract-assigning 2 vectors subtract-assigns all their components.
    macro_rules! sub_assign_test {
        ($vec:ident<$t:ty, $n:literal> { $(
            $a:ident { $([$a_index:literal] => $a_comp:ident: $a_val:literal),* },
            $b:ident { $([$b_index:literal] => $b_comp:ident: $b_val:literal),* },
        )* }) => {
            #[test]
            fn sub_assign() { $(
                let mut $a = $vec::new($($a_val),*);
                let $b = $vec::new($($b_val),*);
                $a -= $b;
                assert_eq!($a, $vec::new($($a_val - $b_val),*));
                $a -= &$b;
                assert_eq!($a, $vec::new($($a_val - $b_val - $b_val),*));
            )* }
        };
    }
    pub(crate) use sub_assign_test;

    /// Creates a test which checks if multiplying 2 vectors multiplies all their components.
    macro_rules! mul_test {
        ($vec:ident<$t:ty, $n:literal> { $(
            $a:ident { $([$a_index:literal] => $a_comp:ident: $a_val:literal),* },
            $b:ident { $([$b_index:literal] => $b_comp:ident: $b_val:literal),* },
        )* }) => {
            #[test]
            fn mul() { $(
                let $a = $vec::new($($a_val),*);
                let $b = $vec::new($($b_val),*);
                assert_eq!( $a *  $b, $vec::new($($a_val * $b_val),*));
                assert_eq!( $a * &$b, $vec::new($($a_val * $b_val),*));
                assert_eq!(&$a *  $b, $vec::new($($a_val * $b_val),*));
                assert_eq!(&$a * &$b, $vec::new($($a_val * $b_val),*));
            )* }
        };
    }
    pub(crate) use mul_test;

    /// Creates a test which checks if multiply-assigning 2 vectors multiply-assigns all their components.
    macro_rules! mul_assign_test {
        ($vec:ident<$t:ty, $n:literal> { $(
            $a:ident { $([$a_index:literal] => $a_comp:ident: $a_val:literal),* },
            $b:ident { $([$b_index:literal] => $b_comp:ident: $b_val:literal),* },
        )* }) => {
            #[test]
            fn mul_assign() { $(
                let mut $a = $vec::new($($a_val),*);
                let $b = $vec::new($($b_val),*);
                $a *= $b;
                assert_eq!($a, $vec::new($($a_val * $b_val),*));
                $a *= &$b;
                assert_eq!($a, $vec::new($($a_val * $b_val * $b_val),*));
            )* }
        };
    }
    pub(crate) use mul_assign_test;
}
