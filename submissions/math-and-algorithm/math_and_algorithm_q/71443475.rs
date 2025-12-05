use num_integer::lcm;
use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [usize; n],
  }

  println!("{}", a.into_iter().reduce(|acc, i| lcm(acc, i)).unwrap());
}
