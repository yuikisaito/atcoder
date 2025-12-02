use proconio::input;

fn main() {
  input! {
    n: usize,
    s: usize,
  }

  let max = n.min(s - 1);

  println!("{}", (max.pow(2) + max) / 2);
}
