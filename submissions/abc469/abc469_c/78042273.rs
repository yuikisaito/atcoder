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

  let mut sum = 0;
  let mut iter = change.iter();
  let mut index = 0;
  for k in 0..n {
    sum += s[k];
    let base = change[k];
    let goal = base - sum;
    eprintln!("{}, {}", sum, goal);
    index += iter.find_position(|x| **x <= goal).map(|x| x.0 + 1).unwrap_or(n);
    println!("{}", n.min(index));
  }
}
