use itertools::Itertools;
use itertools::izip;
use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    _m: usize,
    d: usize,
    s: Chars,
  }

  let from_left = s
    .iter()
    .enumerate()
    .scan(usize::MAX, |g, (i, x)| {
      if *x == 'G' {
        *g = i;
      }
      return Some(g.abs_diff(i) <= d)
    })
    .collect_vec();
  let from_right = s
    .iter()
    .rev()
    .enumerate()
    .scan(usize::MAX, |g, (i, x)| {
      if *x == 'G' {
        *g = i;
      }
      return Some(g.abs_diff(i) <= d)
    })
    .collect_vec();

  eprintln!("{:?}", from_left);
  eprintln!("{:?}", from_right);

  println!("{}", izip!(from_left, from_right.into_iter().rev()).filter(|(x, y)| !(*x || *y)).count());
}
