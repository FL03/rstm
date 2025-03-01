/*
    Appellation: fmt <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

macro_rules! impl_fmt {
    ($wrap:ident($($trait:ident),* $(,)?)) => {
        $(impl<T: core::fmt::$trait> core::fmt::$trait for $wrap<T> {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                core::fmt::$trait::fmt(&self.0, f)
            }
        })*
    };
}

macro_rules! impl_bin_op {
    (@impl $wrap:ident($trait:ident::$method:ident)) => {
        impl<A, B, C> core::ops::$trait<$wrap<B>> for $wrap<A> where A: core::ops::$trait<B, Output = C>{
            type Output = $wrap<C>;

            fn $method(self, rhs: $wrap<B>) -> Self::Output {
                $wrap(core::ops::$trait::$method(self.0, rhs.0))
            }
        }
    };

    ($wrap:ident($($trait:ident::$method:ident),* $(,)?)) => {
        $(impl_bin_op!(@impl $wrap($trait::$method));)*
    };
}

macro_rules! impl_unary_op {
    (@impl $wrap:ident($trait:ident::$method:ident)) => {
        impl<A, B> ::core::ops::$trait for $wrap<A> where A: core::ops::$trait<Output = B>{
            type Output = $wrap<B>;

            fn $method(self) -> Self::Output {
                $wrap(::core::ops::$trait::$method(self.0))
            }
        }
    };

    ($wrap:ident($($trait:ident::$method:ident),* $(,)?)) => {
        $(impl_unary_op!(@impl $wrap($trait::$method));)*
    };
}

macro_rules! impl_assign_op {
    (@impl $wrap:ident($trait:ident::$method:ident)) => {
        impl<A, B> core::ops::$trait<B> for $wrap<A> where A: core::ops::$trait<B> {
            fn $method(&mut self, rhs: B) {
                core::ops::$trait::$method(&mut self.0, rhs)
            }
        }
    };

    ($wrap:ident($($trait:ident::$method:ident),* $(,)?)) => {
        $(impl_assign_op!(@impl $wrap($trait::$method));)*
    };
}
