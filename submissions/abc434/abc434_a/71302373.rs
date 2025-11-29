use proconio::input;

fn main() {
  input! {
    w: usize,
    b: usize,
  }

  println!("{}", (1000 * w) / b + 1);
}
