use proconio::input;

fn main() {
  input! {
    x: usize,
  }

  println!("{}", (1..=3).skip_while(|i| *i == x).next().unwrap());
}
