use proconio::input;

fn main() {
  input! {
    n: usize,
    m: usize,
  }

  println!("{}", if 2 * m - 1 <= n { "Yes" } else { "No" });
}
