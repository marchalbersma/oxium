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
