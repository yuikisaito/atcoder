use std::vec;

use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    n: usize,
  }

  let mut matrix = vec![vec![0; n]; n];

  let mut k = 1;
  let mut r = 0;
  let mut c = (n - 1) / 2;
  matrix[r][c] = k;
  for _ in 1..n * n {
    k += 1;
    let y = (r + n - 1) % n;
    let x = (c + 1) % n;
    if matrix[y][x] == 0 {
      r = y;
      c = x;
      matrix[r][c] = k;
    } else {
      r = (r + 1) % n;
      matrix[r][c] = k;
    }
  }

  for l in matrix {
    println!("{}", l.into_iter().join(" "));
  }
}
