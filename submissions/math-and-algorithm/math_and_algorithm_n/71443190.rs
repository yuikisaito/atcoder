use itertools::Itertools;
use num_integer::sqrt;
use proconio::input;

fn main() {
  input! {
    mut n: usize,
  }

  let mut result = Vec::new();
  for i in 2..=sqrt(n) {
    while n % i == 0 {
      result.push(i);
      n /= i;
    }
  }

  println!("{}", result.into_iter().join(" "));
}
