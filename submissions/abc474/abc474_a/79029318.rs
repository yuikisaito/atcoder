use proconio::input;

fn main() {
  input! {
    x: usize,
  }

  println!("{}", (1..=3).find(|i| *i != x).unwrap());
}
