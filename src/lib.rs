pub use partial_derive2_derive::*;

/// This is implemented on the "Full" struct.
pub trait HasPartial {
  type Partial;
  fn merge_partial(self, partial: Self::Partial) -> Self;
}

/// This is implemented on the "Partial" struct.
/// Provides a method to check whether all fields are None.
pub trait MaybeNone {
  /// Check if all fields are None.
  fn is_none(&self) -> bool;
}

/// This is implemented on the "Full" struct if #[partial(diff)] is specified.
/// Required all fields on the "Full" struct to implement [PartialEq]
pub trait PartialDiff<Partial> {
  /// Diffs a partial against self, returning a partial where all "Some" fields
  /// are not equal to the corresponding field on Self.
  fn partial_diff(&self, partial: Partial) -> Partial;
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
