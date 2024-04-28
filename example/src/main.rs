use partial_derive2::{Partial, PartialDiff};

#[derive(Debug, Partial)]
#[partial_derive(Debug)]
#[partial(diff, from)]
struct User {
  name: Option<String>,
  enabled: bool,
  age: i64,
}

fn main() {
  let user = User {
    name: Some(String::from("uso")),
    enabled: true,
    age: 40,
  };

  let partial = PartialUser {
    name: None,
    enabled: Some(false),
    age: Some(40),
  };

  let diff = user.partial_diff(partial);

  println!("{diff:#?}")
}
