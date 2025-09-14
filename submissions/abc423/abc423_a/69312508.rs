use proconio::input;

fn main() {
  input! {
    x: usize,
    c: usize,
  }

  for y in (0..x + 1).step_by(1000).rev() {
    let result = y as f64 * ((1000 + c) as f64 / 1000.0);
    if result <= x as f64 {
      println!("{}", y);
      return;
    }
  }
}
