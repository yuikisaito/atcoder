use proconio::input;

fn main() {
  input! {
    n: usize,
    abs: [(usize, usize, String); n],
  }

  println!("{}", abs.into_iter().filter(|(_, _, s)| s == "keep").map(|(a, b, _)| b - a).sum::<usize>());
}
