use partial_derive2::{Diff, FieldDiff, Partial, PartialDiff};

#[derive(Debug, Partial)]
#[partial_derive(Debug)]
#[partial(diff, from)]
struct User {
  name: Option<String>,
  desc: String,
  enabled: bool,
  age: i64,
}

fn main() {
  let user = User {
    name: Some(String::from("uso")),
    desc: String::from(""),
    enabled: true,
    age: 40,
  };

  let partial = PartialUser {
    name: Some(String::from("uso2")),
    desc: None,
    enabled: Some(false),
    age: Some(40),
  };

  let diff = user.partial_diff(partial);

  println!("{diff:#?}");

  for FieldDiff { field, from, to } in diff.iter_field_diffs() {
    println!("field: {field} | from: {from} | to: {to}")
  }

  let partial: PartialUser = diff.into();

  println!("{partial:#?}");
}
