extern crate reddish;
use reddish::*;

fn main() {
  println!("{}", snake_case("fooBarBaz"));
  println!("{}", capitalize("foo bar"));
  println!("{}", kebab_case("Foo Bar"));
  println!("{}", title_case("foo bar"));
}
