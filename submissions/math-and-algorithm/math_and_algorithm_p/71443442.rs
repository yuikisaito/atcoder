use num_integer::gcd;
use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [usize; n],
  }

  println!("{}", a.into_iter().reduce(|acc, i| gcd(acc, i)).unwrap());
}
