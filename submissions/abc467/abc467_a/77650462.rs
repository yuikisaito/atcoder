use proconio::input;

fn main() {
  input! {
    h: usize,
    w: usize,
  }

  println!("{}", if 400 * w >= h * h { "Yes" } else { "No" });
}
