use std::collections::HashSet;

use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [isize; n],
  }

  let mut a_iter = a.iter().filter(|&&i| i != -1);
  let a_vec = a_iter.clone().collect_vec();
  let a_hs: HashSet<_> = a_iter.clone().collect();
  if a_hs.len() != a_vec.len() {
    println!("No");
    return;
  }

  if !a_iter.all(|&i| 1 <= n && i as usize <= n) {
    println!("No");
    return;
  }

  let mut not_used = (1..=n as isize).filter(|i| !a.contains(i));

  let result = a
    .iter()
    .map(|&i| if i != -1 { i } else { not_used.next().unwrap() })
    .join(" ");

  println!("Yes");
  println!("{}", result);
}
