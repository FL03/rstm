/*
    Appellation: states <macros>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

macro_rules! getter {
    (@impl $vis:vis $T:ident($($field:tt)*)) => {
        $vis fn into_inner(self) -> Self::Inner {
            self.$($field)*
        }

        $vis const fn get(&self) -> &Self::Inner {
            &self.$($field)*
        }

        $vis fn get_mut(&mut self) -> &mut Self::Inner {
            &mut self.$($field)*
        }

        $vis fn set(&mut self, inner: Self::Inner) {
            self.$($field)* = inner;
        }
    };
    ($($vis:vis $T:ident($($field:tt)*)),* $(,)?) => {
        $(
            getter!(@impl $vis $T($($field)*));
        )*
    };
}

#[macro_export]
macro_rules! state {
    ($state:expr) => {
        $crate::State($state)
    };
}
