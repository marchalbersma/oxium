/// Defines a vector struct.
macro_rules! def_vec_struct {
    ($name:ident<$t:ty, $n:literal> { $($comp:ident),* }) => {
        #[doc = concat!(" A ", $n, "-dimensional vector with [`", stringify!($t), "`] components.")]
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
