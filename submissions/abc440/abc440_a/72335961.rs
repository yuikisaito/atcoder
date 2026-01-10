use proconio::input;

fn main() {
  input! {
    x: usize,
    y: u32,
  }

  println!("{}", x * 2usize.pow(y));
}
