extern crate reddish;
use reddish::{capitalize, kebab_case, pad, pad_end, snake_case, title_case, truncate};

fn main() {
  println!("{}", snake_case("fooBarBaz"));
  // foo_bar_baz
  println!("{}", capitalize("foo bar"));
  // Foo bar
  println!("{}", kebab_case("Foo Bar"));
  // foo-bar
  println!("{}", title_case("foo bar"));
  // Foo bar
  println!("{}", pad("foo bar", 1, None));
  //  foo bar
  println!("{}", pad("foo bar", 1, Some('*')));
  // *foo bar*
  println!("{}", pad_end("foo bar", 1, None));
  // foo bar
  println!("{}", pad_end("foo bar", 1, Some('*')));
  // foo bar*
  println!("{}", truncate("foo bar", 3));
  // foo
}
