/// Defines a matrix struct.
macro_rules! def_mat_struct {
    ($name:ident<$t:ty, $r:literal, $c:literal, $row:ident>) => {
        #[doc = concat!(" A ", $r, " x ", $c, " matrix with [`", stringify!($t), "`] elements.")]
        pub struct $name {
            rows: [$row; $r],
        }
    };
}
pub(crate) use def_mat_struct;

/// Implements the `new()` associated function which creates a matrix from individual elements.
macro_rules! impl_mat_new {
    ($(#[$meta:meta])* $name:ident<$t:ty, $row:ident> { $([$($elem:ident),*],)* }) => {
        $(#[$meta])*
        impl $name {
            /// Creates a new matrix from the given elements.
            pub const fn new($($($elem: $t),*),*) -> Self {
                Self { rows: [$($row::new($($elem),*)),*] }
            }
        }
    };
}
pub(crate) use impl_mat_new;

/// Creates a test which checks if calling the `new()` associated function correctly sets all matrix elements.
#[cfg(test)]
macro_rules! test_mat_new {
    ($name:ident { $($var:ident { $($row:literal: [$($elem:ident: $val:literal),*],)* },)* }) => {
        #[test]
        fn new() { $(
            let $var = $name::new($($($val,)*)*);
            $($(assert_eq!($var.rows[$row].$elem, $val);)*)*
        )* }
    };
}
#[cfg(test)]
pub(crate) use test_mat_new;
