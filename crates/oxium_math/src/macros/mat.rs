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

/// Implements the `from_rows()` associated function which creates a matrix from row vectors.
macro_rules! impl_mat_from_rows {
    ($name:ident<$row:ident> { $($arg:ident),* }) => {
        impl $name {
            /// Creates a new matrix from the given rows.
            pub const fn from_rows($($arg: $row),*) -> Self {
                Self { rows: [$($arg),*] }
            }
        }
    };
}
pub(crate) use impl_mat_from_rows;

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

/// Implements the [`Neg`](std::ops::Neg) trait to perform element-wise matrix negation.
macro_rules! impl_mat_neg {
    ($name:ident { $($r:literal),*}) => {
        /// Perform element-wise matrix negation.
        impl std::ops::Neg for $name {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self::from_rows($(-self.rows[$r]),*)
            }
        }

        // TODO: remove duplicate code by placing a custom attribute macro on the above impl.
        /// Perform element-wise matrix negation.
        impl std::ops::Neg for &$name {
            type Output = $name;

            fn neg(self) -> Self::Output {
                $name::from_rows($(-self.rows[$r]),*)
            }
        }
    };
}
pub(crate) use impl_mat_neg;

/// Implements the [`Add`](std::ops::Add) trait to perform element-wise matrix addition.
macro_rules! impl_mat_add {
    ($name:ident { $($r:literal),* }) => {
        /// Perform element-wise matrix addition.
        impl std::ops::Add<$name> for $name {
            type Output = $name;

            fn add(self, other: $name) -> Self::Output {
                $name::from_rows($(self[$r] + other[$r]),*)
            }
        }

        // TODO: remove duplicate code by placing a custom attribute macro on the first impl.
        /// Perform element-wise matrix addition.
        impl std::ops::Add<&$name> for $name {
            type Output = Self;

            fn add(self, other: &$name) -> Self::Output {
                $name::from_rows($(self[$r] + other[$r]),*)
            }
        }

        // TODO: remove duplicate code by placing a custom attribute macro on the first impl.
        /// Perform element-wise matrix addition.
        impl std::ops::Add<$name> for &$name {
            type Output = $name;

            fn add(self, other: $name) -> Self::Output {
                $name::from_rows($(self[$r] + other[$r]),*)
            }
        }

        // TODO: remove duplicate code by placing a custom attribute macro on the first impl.
        /// Perform element-wise matrix addition.
        impl std::ops::Add<&$name> for &$name {
            type Output = $name;

            fn add(self, other: &$name) -> Self::Output {
                $name::from_rows($(self[$r] + other[$r]),*)
            }
        }
    }
}
pub(crate) use impl_mat_add;

/// Implements the [`AddAssign`](std::ops::AddAssign) trait to perform element-wise matrix addition assignment.
macro_rules! impl_mat_add_assign {
    ($name:ident { $($r:literal),* }) => {
        /// Perform element-wise matrix addition assignment.
        impl std::ops::AddAssign<$name> for $name {
            fn add_assign(&mut self, other: $name) {
                $(self[$r] += other[$r];)*
            }
        }

        // TODO: remove duplicate code by placing a custom attribute macro on the above impl.
        /// Perform element-wise matrix addition assignment.
        impl std::ops::AddAssign<&$name> for $name {
            fn add_assign(&mut self, other: &$name) {
                $(self[$r] += other[$r];)*
            }
        }
    };
}
pub(crate) use impl_mat_add_assign;

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

/// Creates a test which checks if calling the `from_rows()` associated function correctly sets all matrix rows.
#[cfg(test)]
macro_rules! test_mat_from_rows {
    ($name:ident { $($var:ident { $($r:literal: $row:expr,)* },)* }) => {
        #[test]
        fn from_rows() { $(
            let $var = $name::from_rows($($row),*);
            $(assert_eq!($var[$r], $row);)*
        )* }
    }
}
#[cfg(test)]
pub(crate) use test_mat_from_rows;

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

/// Creates a test which checks if negating a matrix negates all its elements.
#[cfg(test)]
macro_rules! test_mat_neg {
    ($name:ident { $($var:ident { $($val:literal,)* } => { $($val_neg:literal,)* },)* }) => {
        #[test]
        fn neg() { $(
            let $var = $name::new($($val),*);
            assert_eq!(-$var, $name::new($($val_neg),*));
            assert_eq!(-&$var, $name::new($($val_neg),*));
        )* }
    };
}
#[cfg(test)]
pub(crate) use test_mat_neg;

/// Creates a test which checks if adding 2 matrices adds all their elements.
#[cfg(test)]
macro_rules! test_mat_add {
    ($name:ident { $($a:ident { $($a_val:literal,)* } + $b:ident { $($b_val:literal,)* } = $c:ident,)* }) => {
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
pub(crate) use test_mat_add;

/// Creates a test which checks if add-assigning 2 matrices add-assigns all their elements.
#[cfg(test)]
macro_rules! test_mat_add_assign {
    ($name:ident { $($a:ident { $($a_val:literal,)* } += $b:ident { $($b_val:literal,)* },)* }) => {
        #[test]
        fn add_assign() { $(
            let mut $a = $name::new($($a_val),*);
            let $b = $name::new($($b_val),*);
            $a += $b;
            assert_eq!($a, $name::new($($a_val + $b_val),*));
            $a += &$b;
            assert_eq!($a, $name::new($($a_val + $b_val + $b_val),*));
        )* }
    };
}
#[cfg(test)]
pub(crate) use test_mat_add_assign;
