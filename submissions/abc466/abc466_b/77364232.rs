use std::collections::HashMap;

use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
  input! {
    n: usize,
    m: usize,
    cs: [(Usize1, isize); n],
  }

  let mut hm = HashMap::new();

  for (c, s) in cs {
    let v = hm.entry(c).or_insert(0);
    if s >= *v {
      *v = s;
    }
  }

  println!("{}", (0..m).map(|k| hm.get(&k).unwrap_or(&-1).to_string()).join(" "));
}
