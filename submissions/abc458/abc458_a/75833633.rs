use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    s: Chars,
    n: usize,
  }

  println!("{}", s.into_iter().rev().skip(n).rev().skip(n).collect::<String>());
}
