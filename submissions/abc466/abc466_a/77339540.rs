use proconio::input;

fn main() {
  input! {
    n: usize,
    x: [isize; n],
  }

  println!("{}", if x.into_iter().all(|i| i < 0) { "Yes" } else { "No" });
}
