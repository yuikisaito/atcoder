use num_integer::lcm;
use proconio::input;

fn main() {
  input! {
      n: usize,
      x: usize,
      y: usize,
  }
  let ans = n / x + n / y - n / lcm(x, y);
  println!("{}", ans);
}
