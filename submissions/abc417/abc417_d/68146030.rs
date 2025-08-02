use std::collections::HashMap;

use proconio::input;

fn main() {
  input! {
    n: usize,
    pab: [(usize, usize, usize); n],
    q: usize,
    x: [usize; q],
  }

  let mut db = HashMap::new();
  for xi in x {
    if db.contains_key(&xi) {
      println!("{}", db[&xi]);
      continue;
    }
    let mut tension = xi;
    for &(p, a, b) in pab.iter() {
      if p >= tension {
        tension += a;
      } else if tension <= b {
        tension = 0;
      } else {
        tension -= b;
      }
    }
    db.insert(xi, tension);
    println!("{}", tension)
  }
}
