use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
  input! {
    n: usize,
    s: Chars,
  }

  let mut f = 0;
  for _ in 0..n {
    while f < n && unsafe { s.get_unchecked(f) == &'o' } {
      f += 1;
    }
    f += 1;
    println!("{}", f.min(n));
  }
}
