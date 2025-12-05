use num_integer::gcd;
use proconio::input;

fn main() {
  input! {
    a: usize,
    b: usize,
  }

  println!("{}", gcd(a, b));
}
