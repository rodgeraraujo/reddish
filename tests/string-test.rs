#[cfg(test)]
mod string_tests {
  use reddish;

  #[test]
  fn test_camel_case() {
    assert_eq!(
      reddish::camel_case("foo_barBaz_QuxQUUX"),
      "FooBarBazQuxQUUX"
    );
    assert_eq!(
      reddish::camel_case("Foo_barBaz_QuxQUUX"),
      "FooBarBazQuxQUUX"
    );
  }

  #[test]
  fn capitalize_test() {
    assert_eq!(reddish::capitalize("FOO BAR"), "Foo bar");
    assert_eq!(reddish::capitalize("foo bar"), "Foo bar");
    assert_eq!(reddish::capitalize("--foo--bar--"), "--foo--bar--");
    assert_eq!(reddish::capitalize("__FOO_BAR__"), "__foo_bar__");
  }

  #[test]
  fn kebab_case_test() {
    assert_eq!(reddish::kebab_case("Foo Bar"), "foo-bar");
    assert_eq!(reddish::kebab_case("--foo--bar--"), "foo-bar");
    assert_eq!(reddish::kebab_case("fooBar"), "foo-bar");
    assert_eq!(reddish::kebab_case("FooBar"), "foo-bar");
    assert_eq!(reddish::kebab_case("__FOO_BAR__"), "foo-bar");
    assert_eq!(reddish::kebab_case("-_fOO_-BaR_-"), "f-oo-ba-r");
  }

  #[test]
  fn test_snake_case() {
    assert_eq!(
      reddish::snake_case("foo_barBaz_QuxQUUX"),
      "foo_bar_baz_qux_quux"
    );
    assert_eq!(
      reddish::snake_case("Foo_barBaz_QuxQUUX"),
      "foo_bar_baz_qux_quux"
    );
  }

  #[test]
  fn title_case_test() {
    assert_eq!(reddish::title_case("foo-bar_baz-QUAX"), "Foo-bar_baz-quax");
    assert_eq!(reddish::title_case("foo bar baz QUAX"), "Foo bar baz quax");
    assert_eq!(reddish::title_case("FOO BAR"), "Foo bar");
    assert_eq!(reddish::title_case("foo bar"), "Foo bar");
  }

  #[test]
  fn test_pad() {
    assert_eq!(reddish::pad("Foo Bar", 0, None), "Foo Bar");
    assert_eq!(reddish::pad("Foo Bar", 1, None), " Foo Bar ");
    assert_eq!(reddish::pad("Foo Bar", 2, None), "  Foo Bar  ");
    assert_eq!(reddish::pad("Foo Bar", 3, None), "   Foo Bar   ");

    assert_eq!(reddish::pad("Foo Bar", 0, Some('*')), "Foo Bar");
    assert_eq!(reddish::pad("Foo Bar", 1, Some('*')), "*Foo Bar*");
    assert_eq!(reddish::pad("Foo Bar", 2, Some('*')), "**Foo Bar**");
    assert_eq!(reddish::pad("Foo Bar", 3, Some('*')), "***Foo Bar***");
  }

  #[test]
  fn test_pad_end() {
    assert_eq!(reddish::pad_end("Foo Bar", 0, None), "Foo Bar");
    assert_eq!(reddish::pad_end("Foo Bar", 1, None), "Foo Bar ");
    assert_eq!(reddish::pad_end("Foo Bar", 2, None), "Foo Bar  ");
    assert_eq!(reddish::pad_end("Foo Bar", 3, None), "Foo Bar   ");

    assert_eq!(reddish::pad_end("Foo Bar", 0, Some('*')), "Foo Bar");
    assert_eq!(reddish::pad_end("Foo Bar", 1, Some('*')), "Foo Bar*");
    assert_eq!(reddish::pad_end("Foo Bar", 2, Some('*')), "Foo Bar**");
    assert_eq!(reddish::pad_end("Foo Bar", 3, Some('*')), "Foo Bar***");
  }

  #[test]
  fn test_truncate() {
    assert_eq!(reddish::truncate("Foo", 0), "Foo");
    assert_eq!(reddish::truncate("Foo", 1), "Fo");
    assert_eq!(reddish::truncate("Foo", 2), "F");
    assert_eq!(reddish::truncate("Foo", 3), "");
    assert_eq!(reddish::truncate("Foo", 4), "");
  }
}
