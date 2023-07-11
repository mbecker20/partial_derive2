#[macro_export]
macro_rules! is_option {
    (Option<$ty:ty>) => {
        true
    };
    ($ty:ty) => {
        false
    };
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
