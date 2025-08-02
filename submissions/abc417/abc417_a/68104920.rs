use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    n: usize,
    a: usize,
    b: usize,
    s: Chars,
  }

  println!("{}", s[a..n - b].into_iter().collect::<String>());
}
