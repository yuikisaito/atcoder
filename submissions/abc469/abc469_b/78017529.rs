use std::iter::once;

use itertools::Itertools;
use itertools::izip;
use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    n: usize,
    s: Chars,
  }

  let mut table = Vec::new();
  table.push(s.into_iter().map(|i| i == 'o').collect_vec());
  table.push(once(false).chain(table[0].clone().into_iter().take(n - 1)).collect_vec());
  table.push(table[0].clone().into_iter().skip(1).chain(once(false)).collect_vec());

  println!("{}", izip!(table[0].clone(), table[1].clone(), table[2].clone()).filter(|&(x, y, z)| !x && !y && !z).count());
}
