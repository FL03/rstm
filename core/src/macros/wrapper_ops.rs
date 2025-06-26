macro_rules! wrapper_impl_binary {
    (@impl $wrap:ident($trait:ident::$method:ident)) => {
        impl<A, B, C> ::core::ops::$trait<$wrap<B>> for $wrap<A>
        where
            A: ::core::ops::$trait<B, Output = C>,
        {
            type Output = $wrap<C>;

            fn $method(self, rhs: $wrap<B>) -> Self::Output {
                $wrap(::core::ops::$trait::$method(self.0, rhs.0))
            }
        }

        impl<'a, A, B, C> ::core::ops::$trait<&'a $wrap<B>> for $wrap<A>
        where
            A: ::core::ops::$trait<&'a B, Output = C>,
        {
            type Output = $wrap<C>;

            fn $method(self, rhs: &'a $wrap<B>) -> Self::Output {
                $wrap(::core::ops::$trait::$method(self.0, &rhs.0))
            }
        }

        impl<'a, A, B, C> ::core::ops::$trait<&'a $wrap<B>> for &'a $wrap<A>
        where
            &'a A: ::core::ops::$trait<&'a B, Output = C>,
        {
            type Output = $wrap<C>;

            fn $method(self, rhs: &'a $wrap<B>) -> Self::Output {
                $wrap(::core::ops::$trait::$method(&self.0, &rhs.0))
            }
        }

        impl<'a, A, B, C> ::core::ops::$trait<$wrap<B>> for &'a $wrap<A>
        where
            &'a A: ::core::ops::$trait<B, Output = C>,
        {
            type Output = $wrap<C>;

            fn $method(self, rhs: $wrap<B>) -> Self::Output {
                $wrap(::core::ops::$trait::$method(&self.0, rhs.0))
            }
        }
    };
    ($wrap:ident(
        $(
            $trait:ident::$method:ident
        ),* $(,)?
    )) => {
        paste::paste! {
            $(
                wrapper_impl_binary!(@impl $wrap($trait::$method));
                wrapper_impl_assign!(@impl $wrap([<$trait Assign>]::[<$method _assign>]));
            )*
        }
    };
}

macro_rules! wrapper_impl_assign {
    (@impl $wrap:ident($trait:ident::$method:ident)) => {
        impl<A, B> ::core::ops::$trait<B> for $wrap<A>
        where
            A: ::core::ops::$trait<B>,
        {
            fn $method(&mut self, rhs: B) {
                ::core::ops::$trait::$method(&mut self.0, rhs)
            }
        }

        impl<A, B> ::core::ops::$trait<B> for &mut $wrap<A>
        where
            A: ::core::ops::$trait<B>,
        {
            fn $method(&mut self, rhs: B) {
                ::core::ops::$trait::$method(&mut self.0, rhs)
            }
        }
    };

    ($wrap:ident($($trait:ident::$method:ident),* $(,)?)) => {
        $(wrapper_impl_assign!(@impl $wrap($trait::$method));)*
    };
}

macro_rules! wrapper_impl_unary {
    (@impl $wrap:ident($trait:ident::$method:ident)) => {
        impl<A, B> ::core::ops::$trait for $wrap<A>
        where
            A: ::core::ops::$trait<Output = B>,
        {
            type Output = $wrap<B>;

            fn $method(self) -> Self::Output {
                $wrap(::core::ops::$trait::$method(self.0))
            }
        }

        impl<'a, A, B> ::core::ops::$trait for &'a $wrap<A>
        where
            &'a A: ::core::ops::$trait<Output = B>,
        {
            type Output = $wrap<B>;

            fn $method(self) -> Self::Output {
                $wrap(::core::ops::$trait::$method(&self.0))
            }
        }

        impl<'a, A, B> ::core::ops::$trait for &'a mut $wrap<A>
        where
            &'a A: ::core::ops::$trait<Output = B>,
        {
            type Output = $wrap<B>;

            fn $method(self) -> Self::Output {
                $wrap(::core::ops::$trait::$method(&self.0))
            }
        }
    };

    ($wrap:ident($($trait:ident::$method:ident),* $(,)?)) => {
        $(wrapper_impl_unary!(@impl $wrap($trait::$method));)*
    };
}
