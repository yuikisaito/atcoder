use itertools::izip;
use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    _n: usize,
    s: Chars,
    t: Chars,
  }

  println!("{}", if izip!(s, t).into_iter().any(|(a, b)| b != '*' && a != b) { "No" } else { "Yes" });
}
