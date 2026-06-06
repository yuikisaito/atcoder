use proconio::input;
use proconio::marker::Usize1;

fn main() {
  input! {
    n: usize,
    a: [Usize1; n],
    b: [Usize1; n],
  }

  println!("{}", if a.into_iter().enumerate().all(|(i, ono)| i == b[ono]) { "Yes" } else { "No" });
}
