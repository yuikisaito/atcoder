use proconio::input;

fn main() {
  input! {
    a: usize,
    d: usize,
  }

  println!("{}", if a <= d { "Yes" } else { "No" });
}
