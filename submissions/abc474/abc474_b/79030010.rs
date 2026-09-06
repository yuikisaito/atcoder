use proconio::input;
use proconio::marker::Usize1;

fn main() {
  input! {
    n: usize,
    p: [Usize1; n],
  }

  println!("{}", if p.into_iter().enumerate().any(|(i, j)| j >= (i + 1).div_ceil(10) * 10) { "No" } else { "Yes" });
}
