use num_integer::sqrt;
use proconio::input;

fn main() {
  input! {
    n: usize,
  }

  for i in 2..=sqrt(n) {
    if n % i == 0 {
      println!("No");
      return
    }
  }

  println!("Yes");
}
