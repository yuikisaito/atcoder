use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    n: usize,
    q: usize,
    xy: [(usize, usize); q],
  }

  let mut versions = (1..=n).collect_vec();
  for i in 0..q {
    let mut cnt = 0;
    for j in 0..n {
      if versions[j] <= xy[i].0 {
        cnt += 1;
        versions[j] = xy[i].1;
      }
    }
    println!("{}", cnt);
  }
}
