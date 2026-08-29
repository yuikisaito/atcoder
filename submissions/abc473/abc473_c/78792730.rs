use std::iter::repeat;

use indexmap::IndexMap;
use itertools::izip;
use proconio::{input, marker::Usize1};

fn main() {
  input! {
    n: usize,
    k: usize,
    a: [Usize1; n],
  }

  let mut map = IndexMap::<usize, usize>::from_iter(izip!(0..k, repeat(5)));
  let mut max = 0;

  for k in a {
    map
      .entry(k)
      .and_modify(|v| {
        *v += 1;
        max = max.max(*v);
      })
      .or_insert(1);
  }

  println!("{}", map.into_iter().filter(|&(_, v)| max - 1 <= v && v <= max).map(|(k, _)| k).count());
}
