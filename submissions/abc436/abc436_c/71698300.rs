use std::collections::HashSet;
use std::convert::TryInto;

use proconio::input;

fn main() {
  input! {
    n: usize,
    m: usize,
    rc: [(isize, isize); m],
  }

  let mut hs = HashSet::new();
  for (r, c) in rc.into_iter().map(|(y, x)| (y - 1, x - 1)) {
    if (-1..=1).all(|y| {
      (-1..=1).all(|x| {
        if 0 <= r + y
          && r + y < n.try_into().unwrap()
          && 0 <= c + x
          && c + x < n.try_into().unwrap()
        {
          let i: usize = (r + y).try_into().unwrap();
          let j: usize = (c + x).try_into().unwrap();
          !hs.contains(&(i, j))
        } else {
          true
        }
      })
    }) {
      let i: usize = r.try_into().unwrap();
      let j: usize = c.try_into().unwrap();
      hs.insert((i, j));
    }
  }

  println!("{}", hs.len());
}
