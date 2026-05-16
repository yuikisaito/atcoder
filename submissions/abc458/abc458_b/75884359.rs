use itertools::Itertools;
use nalgebra::DMatrix;
use proconio::input;

fn main() {
  input! {
    h: usize,
    w: usize,
  }

  let u = DMatrix::from_fn(h, w, |y, _| (y > 0) as usize);
  let d = DMatrix::from_fn(h, w, |y, _| (y < h - 1) as usize);
  let l = DMatrix::from_fn(h, w, |_, x| (x > 0) as usize);
  let r = DMatrix::from_fn(h, w, |_, x| (x < w - 1) as usize);
  let sum = u + d + l + r;

  for y in 0..h {
    println!("{}", sum.row(y).into_iter().join(" "));
  }
}
