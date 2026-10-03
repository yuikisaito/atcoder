use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
  input! {
    n: usize,
    m: usize,
  }

  for _ in 0..m % n {
    println!("{}", m / n + 1)
  }
  for _ in m % n..n {
    println!("{}", m / n)
  }
}
