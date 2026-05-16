use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    h: usize,
    w: usize,
  }

  for y in 0..h {
    println!("{}", (0..w).map(|x| (y > 0) as usize + (y < h - 1) as usize + (x > 0) as usize + (x < w - 1) as usize).join(" "));
  }
}
