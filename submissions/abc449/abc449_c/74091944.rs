use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    n: usize,
    l: usize,
    r: usize,
    s: Chars,
  }

  println!("{}", (0..n).map(|k| (l..=r).filter(|&j| k + j < n && s[k] == s[k + j]).count()).sum::<usize>());
}
