use proconio::input;

fn main() {
  input! {
    n: usize,
    s: usize,
  }

  let max = n.min(s - 1) as f64;

  println!("{}", 1.0 / 2.0 * max * (1.0 + max));
}
