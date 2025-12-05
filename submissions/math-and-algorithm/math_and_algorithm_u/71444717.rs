use permutohedron::factorial;
use proconio::input;

fn main() {
  input! {
    n: usize,
    r: usize,
  }

  println!("{}", factorial(n) / factorial(n - r) / factorial(r));
}
