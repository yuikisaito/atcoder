use proconio::input;

fn main() {
  input! {
    h: usize,
    w: usize,
  }

  println!("{}", if 10000 * w >= 25 * h.pow(2) { "Yes" } else { "No" });
}
