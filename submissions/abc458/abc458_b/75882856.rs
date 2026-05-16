use itertools::Itertools;
use nalgebra::DMatrix;
use proconio::input;

fn main() {
  input! {
    h: usize,
    w: usize,
  }

  let base = DMatrix::repeat(h, w, 2);
  let row = DMatrix::from_fn(h, w, |y, _| (1..h - 1).contains(&y) as usize);
  let col = DMatrix::from_fn(h, w, |_, x| (1..w - 1).contains(&x) as usize);
  let sum = base + row + col;

  for y in 0..h {
    println!("{}", sum.row(y).into_iter().join(" "));
  }
}
