use partial_derive2::{Diff, FieldDiff, Partial, PartialDiff};

#[derive(Debug, Partial)]
#[partial_derive(Debug)]
#[partial(diff, from)]
struct User {
  name: Option<String>,
  desc: String,
  enabled: bool,
  age: i64,
  things: Vec<Thing>,
}

#[derive(Debug, Clone, PartialEq)]
struct Thing {
  ab: String,
  cd: bool,
}

fn main() {
  let user = User {
    name: Some(String::from("uso")),
    desc: String::from(""),
    enabled: true,
    age: 40,
    things: vec![
      Thing {
        ab: String::from("coolio1"),
        cd: true,
      },
      Thing {
        ab: String::from("coolio1"),
        cd: true,
      },
      Thing {
        ab: String::from("coolio1"),
        cd: true,
      },
      Thing {
        ab: String::from("coolio1"),
        cd: true,
      },
      Thing {
        ab: String::from("coolio1"),
        cd: true,
      },
    ],
  };

  let partial = PartialUser {
    name: Some(String::from("uso2")),
    desc: None,
    enabled: Some(false),
    age: Some(40),
    things: Some(vec![
      Thing {
        ab: String::from("coolio1"),
        cd: true,
      },
      Thing {
        ab: String::from("coolio1"),
        cd: true,
      },
      Thing {
        ab: String::from("coolio1"),
        cd: true,
      },
      Thing {
        ab: String::from("coolio3"),
        cd: true,
      },
      Thing {
        ab: String::from("coolio1"),
        cd: true,
      },
    ]),
  };

  let diff = user.partial_diff(partial);

  println!("{diff:#?}");

  for FieldDiff { field, from, to } in diff.iter_field_diffs() {
    println!("field: {field} | from: {from} | to: {to}")
  }

  let partial: PartialUser = diff.into();

  println!("{partial:#?}");
}
