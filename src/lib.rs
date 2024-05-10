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
/// Requires all fields on the "Full" struct to implement [PartialEq]
pub trait PartialDiff<P, D: Diff + Into<P>> {
  /// Diffs a partial against self, returning a partial where all "Some" fields
  /// are not equal to the corresponding field on Self.
  fn partial_diff(&self, partial: P) -> D;

  fn minimize_partial(&self, partial: P) -> P {
    self.partial_diff(partial).into()
  }
}

#[derive(Debug)]
pub struct FieldDiff {
  /// The name of diffed field
  pub field: &'static str,
  /// The previous value Debug formatted
  pub from: String,
  /// The current value Debug formatted
  pub to: String,
}

/// This is implemented on the "Diff" struct if #[partial(diff)] is specified.
/// Requires all fields on the "Full" struct to implement [std::fmt::Debug]
pub trait Diff {
  fn iter_field_diffs(&self) -> impl Iterator<Item = FieldDiff>;

  // /// Takes a formatting function for lines to push for each field.
  // /// The formatting function consumes the field name and debug formatted prev and curr values.
  // /// format_field: (field_name, prev_value, curr_value) -> line
  // fn format_diff(&self, format_field: impl Fn(&'static str, String, String) -> String) -> String;

  // fn format_diff_default(&self) -> String {
  //   self.format_diff(default_diff_formatter)
  // }
}

// pub fn default_diff_formatter(field: &'static str, prev: String, curr: String) -> String {
//   format!("{field}: {prev} => {curr} \n")
// }

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

#[macro_export]
macro_rules! value_maybe_as_vec {
  (Vec<$ty:ty>, $_:expr, $expr:expr) => {
    $expr
  };
  ($ty:ty, $expr:expr, $_:expr) => {
    $expr
  };
}
