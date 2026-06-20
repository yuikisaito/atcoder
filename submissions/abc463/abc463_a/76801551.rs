use proconio::input;

fn main() {
  input! {
    x: usize,
    y: usize,
  }

  println!("{}", if x as f64 / y as f64 == 16. / 9. { "Yes" } else { "No" });
}
