use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::iter::FromIterator;

use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    n: usize,
    hl: [(usize, usize); n],
    q: usize,
    t: [usize; q],
  }

  let mut highest = 0;
  let mut height = Vec::new();
  let mut time = Vec::new();
  for (h, l) in hl.into_iter().sorted_by_key(|&(_, l)| -(l as isize)) {
    if h > highest {
      height.push(h);
      time.push(l);
      highest = h;
    }
  }

  let bs = BTreeSet::from_iter(t.clone().into_iter());
  let mut iter = time.iter();
  let mut ans = BTreeMap::new();
  let mut i = 0;
  for a in bs.into_iter().rev() {
    i += iter.take_while_ref(|&&l| a < l).count();
    ans.insert(a, height[i - 1]);
  }

  for a in t {
    println!("{}", ans.get(&a).unwrap());
  }
}
