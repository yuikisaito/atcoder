use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    n: usize,
    s: Chars,
  }

  let s = s.into_iter().map(|c| (c == 'o') as isize).collect_vec();
  let change = s
    .iter()
    .scan(0isize, |state, i| {
      *state += i - 1;
      Some(*state)
    })
    .collect_vec();

  for k in 1..=n {
    let have: isize = s[0..k].iter().sum();
    let mut iter = change.iter();
    let base = iter.nth(k - 1).unwrap();
    let goal = base - have;
    println!("{}", iter.position(|x| *x <= goal).map(|x| x + k + 1).unwrap_or(n));
  }
}
