extern crate reddish;
use reddish::*;

fn main() {
  println!("{}", snake_case("fooBarBaz"));
  println!("{}", capitalize("foo bar"));
  println!("{}", kebab_case("Foo Bar"));
  println!("{}", title_case("foo bar"));

  println!("{}", pad("foo bar", 1, None));
  println!("{}", pad("foo bar", 1, Some('*')));

  println!("{}", pad_end("foo bar", 1, None));
  println!("{}", pad_end("foo bar", 1, Some('*')));

  println!("{}", truncate("foo bar", 3));
}
