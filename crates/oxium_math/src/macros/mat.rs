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
