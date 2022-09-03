/// Defines a vector struct.
macro_rules! def_vec_struct {
    ($name:ident<$t:ty, $n:literal> { $($comp:ident),* }) => {
        #[doc = concat!(" A ", $n, "-dimensional vector with [`", stringify!($t), "`] components.")]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub struct $name { $(
            #[doc = concat!(" The ", stringify!($comp), "-component of the vector.")]
            pub $comp: $t,
        )* }
    };
}
pub(crate) use def_vec_struct;

/// Implements the `new()` associated function which creates a vector from individual components.
macro_rules! impl_vec_new {
    ($name:ident<$t:ty> { $($comp:ident),* }) => {
        impl $name {
            /// Creates a new vector from the given components.
            pub const fn new($($comp: $t),*) -> Self {
                Self { $($comp),* }
            }
        }
    };
}
pub(crate) use impl_vec_new;

/// Implements the [`Index`](std::ops::Index) and [`IndexMut`](std::ops::IndexMut) traits to access vector components by 0-based index.
macro_rules! impl_vec_index {
    ($name:ident<$t:ty, $n:literal> { $($index:literal => $comp:ident),* }) => {
        /// Access vector components by 0-based index.
        impl std::ops::Index<usize> for $name {
            type Output = $t;

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
        /// Access vector components mutably by 0-based index.
        impl std::ops::IndexMut<usize> for $name {
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
pub(crate) use impl_vec_index;

/// Implements the [`Neg`](std::ops::Neg) trait to perform component-wise vector negation.
macro_rules! impl_vec_neg {
    ($name:ident { $($comp:ident),*}) => {
        /// Perform component-wise vector negation.
        impl std::ops::Neg for $name {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self::new($(-self.$comp),*)
            }
        }

        // TODO: remove duplicate code by placing a custom attribute macro on the above impl.
        /// Perform component-wise vector negation.
        impl std::ops::Neg for &$name {
            type Output = $name;

            fn neg(self) -> Self::Output {
                $name::new($(-self.$comp),*)
            }
        }
    };
}
pub(crate) use impl_vec_neg;

/// Implements the [`Add`](std::ops::Add) trait to perform component-wise vector addition.
macro_rules! impl_vec_add {
    ($name:ident { $($comp:ident),* }) => {
        /// Perform component-wise vector addition.
        impl std::ops::Add<$name> for $name {
            type Output = $name;

            fn add(self, other: $name) -> Self::Output {
                $name::new($(self.$comp + other.$comp),*)
            }
        }

        // TODO: remove duplicate code by placing a custom attribute macro on the first impl.
        /// Perform component-wise vector addition.
        impl std::ops::Add<&$name> for $name {
            type Output = Self;

            fn add(self, other: &$name) -> Self::Output {
                $name::new($(self.$comp + other.$comp),*)
            }
        }

        // TODO: remove duplicate code by placing a custom attribute macro on the first impl.
        /// Perform component-wise vector addition.
        impl std::ops::Add<$name> for &$name {
            type Output = $name;

            fn add(self, other: $name) -> Self::Output {
                $name::new($(self.$comp + other.$comp),*)
            }
        }

        // TODO: remove duplicate code by placing a custom attribute macro on the first impl.
        /// Perform component-wise vector addition.
        impl std::ops::Add<&$name> for &$name {
            type Output = $name;

            fn add(self, other: &$name) -> Self::Output {
                $name::new($(self.$comp + other.$comp),*)
            }
        }
    }
}
pub(crate) use impl_vec_add;

/// Creates a test which checks if calling the `new()` associated function correctly sets all vector components.
#[cfg(test)]
macro_rules! test_vec_new {
    ($name:ident { $($var:ident { $($comp:ident: $val:literal),* },)* }) => {
        #[test]
        fn new() { $(
            let $var = $name::new($($val),*);
            $(assert_eq!($var.$comp, $val);)*
        )* }
    };
}
#[cfg(test)]
pub(crate) use test_vec_new;

/// Creates a test which checks if accessing vector components by 0-based index returns the correct components.
#[cfg(test)]
macro_rules! test_vec_index {
    ($name:ident { $($var:ident { $($index:literal => $comp:ident: $val:literal = $val_mut:literal),* },)* }) => {
        #[test]
        fn index() { $(
            let $var = $name::new($($val),*);
            $(assert_eq!($var[$index], $var.$comp);)*
        )* }

        #[test]
        fn index_mut() { $(
            let mut $var = $name::new($($val),*);
            $($var[$index] = $val_mut;)*
            $(assert_eq!($var.$comp, $val_mut);)*
        )* }
    };
}
#[cfg(test)]
pub(crate) use test_vec_index;

/// Creates a test which checks if attempting to access an out of bounds vector component panics.
#[cfg(test)]
macro_rules! test_vec_index_out_of_bounds {
    ($name:ident { $($val:literal),* } $index:literal = $val_mut:literal) => {
        #[test]
        #[should_panic(expected = "index out of bounds")]
        fn index_out_of_bounds() {
            let vec = $name::new($($val),*);
            vec[$index];
        }

        #[test]
        #[should_panic(expected = "index out of bounds")]
        fn index_mut_out_of_bounds() {
            let mut vec = $name::new($($val),*);
            vec[$index] = $val_mut;
        }
    };
}
#[cfg(test)]
pub(crate) use test_vec_index_out_of_bounds;

/// Creates a test which checks if negating a vector negates all its components.
#[cfg(test)]
macro_rules! test_vec_neg {
    ($name:ident { $($var:ident { $($val:literal),* } => { $($val_neg:literal),* },)* }) => {
        #[test]
        fn neg() { $(
            let $var = $name::new($($val),*);
            assert_eq!(-$var, $name::new($($val_neg),*));
            assert_eq!(-&$var, $name::new($($val_neg),*));
        )* }
    };
}
#[cfg(test)]
pub(crate) use test_vec_neg;

/// Creates a test which checks if adding 2 vectors adds all their components.
#[cfg(test)]
macro_rules! test_vec_add {
    ($name:ident { $($a:ident { $($a_val:literal),* } + $b:ident { $($b_val:literal),* } = $c:ident,)* }) => {
        #[test]
        fn add() { $(
            let $a = $name::new($($a_val),*);
            let $b = $name::new($($b_val),*);
            let $c = $name::new($($a_val + $b_val),*);
            assert_eq!($a + $b, $c);
            assert_eq!($a + &$b, $c);
            assert_eq!(&$a + $b, $c);
            assert_eq!(&$a + &$b, $c);
        )* }
    };
}
#[cfg(test)]
pub(crate) use test_vec_add;
