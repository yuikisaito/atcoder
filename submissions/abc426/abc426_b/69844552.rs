use std::collections::HashSet;

use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    s: Chars,
  }

  let types: HashSet<&char> = s.iter().collect();
  for t in types {
    if s.iter().filter(|&si| si == t).count() == 1 {
      println!("{}", s[0]);
      return;
    }
  }
}
