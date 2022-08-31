/// Defines a matrix struct.
macro_rules! def_mat_struct {
    ($name:ident<$t:ty, $r:literal, $c:literal, $row:ident>) => {
        #[doc = concat!(" A ", $r, " x ", $c, " matrix with [`", stringify!($t), "`] elements.")]
        #[derive(Clone, Copy, Debug, PartialEq)]
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

/// Implements the [`Index`](std::ops::Index) and [`IndexMut`](std::ops::IndexMut) traits to access matrix rows by 0-based index.
macro_rules! impl_mat_index {
    ($name:ident<$n:literal, $t:ty> { $($index:literal),* }) => {
        /// Access matrix rows by 0-based index.
        impl std::ops::Index<usize> for $name {
            type Output = $t;

            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    $($index => &self.rows[$index],)*
                    _ => {
                        panic!("index out of bounds: the matrix has {} rows but the index is {}", $n, index)
                    }
                }
            }
        }

        // TODO: remove duplicate code by placing a custom attribute macro on the above impl.
        /// Access matrix rows mutably by 0-based index.
        impl std::ops::IndexMut<usize> for $name {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                match index {
                    $($index => &mut self.rows[$index],)*
                    _ => {
                        panic!("index out of bounds: the matrix has {} rows but the index is {}", $n, index)
                    }
                }
            }
        }
    };
}
pub(crate) use impl_mat_index;

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

/// Creates a test which checks if accessing matrix elements by 0-based index returns the correct elements.
#[cfg(test)]
macro_rules! test_mat_index {
    ($name:ident<$row:ident> { $($var:ident { $($r:literal: [$($val:literal),*] = [$($mut_val:literal),*],)* },)* }) => {
        #[test]
        fn index() { $(
            let $var = $name::new($($($val),*),*);
            $(assert_eq!($var[$r], $row::new($($val),*));)*
        )* }

        #[test]
        fn index_mut() { $(
            let mut $var = $name::new($($($val),*),*);
            $($var[$r] = $row::new($($mut_val),*);)*
            $(assert_eq!($var[$r], $row::new($($mut_val),*));)*
        )* }
    };
}
#[cfg(test)]
pub(crate) use test_mat_index;

/// Creates a test which checks if attempting to access an out of bounds matrix row panics.
#[cfg(test)]
macro_rules! test_mat_index_out_of_bounds {
    ($name:ident<$row:ident> { $([$($val:literal),*],)* } $index:literal = [$($val_mut:literal),*]) => {
        #[test]
        #[should_panic(expected = "index out of bounds")]
        fn index_out_of_bounds() {
            let mat = $name::new($($($val),*),*);
            mat[$index];
        }

        #[test]
        #[should_panic(expected = "index out of bounds")]
        fn index_mut_out_of_bounds() {
            let mut mat = $name::new($($($val),*),*);
            mat[$index] = $row::new($($val_mut),*);
        }
    };
}
#[cfg(test)]
pub(crate) use test_mat_index_out_of_bounds;
