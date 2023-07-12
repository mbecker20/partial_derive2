pub use partial_derive2_derive::*;

pub trait HasPartial {
    type Partial;
    fn merge_partial(self, partial: Self::Partial) -> Self;
    fn merge_partial_in_place(&mut self, partial: Self::Partial);
}

#[macro_export]
macro_rules! make_option {
    (Option<$ty:ty>) => {
        Option<$ty>
    };
    ($ty:ty) => {
        Option<$ty>
    }
}

#[macro_export]
macro_rules! value_as_option {
    (Option<$ty:ty>, $expr:expr) => {
        $expr
    };
    ($ty:ty, $expr:expr) => {
        Some($expr)
    };
}

#[macro_export]
macro_rules! value_maybe_as_option {
    (Option<$ty:ty>, $_:expr, $expr:expr) => {
        $expr
    };
    ($ty:ty, $expr:expr, $_:expr) => {
        $expr
    };
}
