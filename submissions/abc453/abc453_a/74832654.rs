use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    _: usize,
    s: Chars,
  }

  println!("{}", s.into_iter().skip_while(|&c| c == 'o').join(""));
}
