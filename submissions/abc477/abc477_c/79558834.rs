use itertools::Itertools;
use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

use crate::Status::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Status {
  NotScanned,
  Found(usize),
  NotFound,
}

#[fastout]
fn main() {
  input! {
    q: usize,
    s: Chars,
    t: Chars,
  }

  let w = s.len();
  let h = t.len();
  let mut matrix = vec![vec![NotScanned; w + 1]; h + 1];
  matrix[0] = (0..w).map(|i| Found(i)).collect_vec();

  for _ in 0..q {
    input! {
      l: usize,
      r: usize,
    }

    for (y, &c) in t.iter().enumerate().map(|(i, j)| (i + 1, j)) {
      for x in l..=r {
        if let Found(i) = matrix[y - 1][x - 1] {
          matrix[y][x] = if c == s[x - 1] { Found(i) } else { NotFound }
        } else if NotFound == matrix[y - 1][x - 1] {
          matrix[y][x] = NotFound;
        }
      }
    }

    println!("{}", if matrix[h][l..=r].iter().any(|&i| if let Found(j) = i { (l - 1..=r - 1).contains(&j) } else { false }) { "Yes" } else { "No" });
  }
}
