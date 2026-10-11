use std::cmp::Ordering::Greater;
use std::cmp::Ordering::Less;
use std::collections::VecDeque;

use itertools::Itertools;
use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
  input! {
    n: usize,
    q: usize,
    l: [usize; n-1],
    mut r: [usize; n-1],
  }

  let mut l = VecDeque::from(l);
  l.push_front(usize::MAX);
  r.push(usize::MAX);

  let mut rl = l
    .iter_mut()
    .rev()
    .scan(usize::MAX, |state, i| {
      *state = *state.min(i);
      Some(*state)
    })
    .collect_vec();
  rl.reverse();

  let lr = r
    .iter_mut()
    .scan(usize::MAX, |state, i| {
      *state = *state.min(i);
      Some(*state)
    })
    .collect_vec();

  for _ in 0..q {
    input! {
      s: Usize1,
      t: Usize1,
    }

    println!(
      "{}",
      match t.cmp(&s) {
        Less => l[s].min(r[s].saturating_add(rl[s])),
        Greater => r[s].min(l[s].saturating_add(lr[s])),
        _ => unreachable!(),
      }
    );
  }
}
