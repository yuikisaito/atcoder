use proconio::input;

fn main() {
  input! {
    d: usize,
    f: usize,
  }

  println!("{}", ((f + 7 * (d / 7 + 1)) - d) % 7);
}
