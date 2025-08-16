use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    n: usize,
    m: usize,
    s: Chars,
    t: Chars,
    lr: [(usize, usize); m],
  }

  let dic = [s, t].concat();

  let mut order = (0..n * 2).collect_vec();

  for &(l, r) in lr.iter() {
    for i in l - 1..r {
      order.swap(i, i + n);
    }
  }

  for c in order[0..n].iter() {
    print!("{}", dic[*c]);
  }
}
