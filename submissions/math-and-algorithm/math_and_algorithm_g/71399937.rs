use proconio::input;

fn main() {
  input! {
    n: usize,
    x: usize,
    y: usize,
  }

  println!("{}", n / x + n / y - n / (x * y));
}
