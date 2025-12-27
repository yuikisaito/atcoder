use proconio::input;

fn main() {
  input! {
    d: usize,
    f: usize,
  }

  println!(
    "{}",
    (1..=7).filter(|i| (d + i - f) % 7 == 0).next().unwrap()
  );
}
