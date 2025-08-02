use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [usize; n],
  }

  let mut cnt = 0;
  for (i, j) in (0..n).tuple_combinations() {
    if j - i == a[i] + a[j] {
      cnt += 1;
    }
  }

  println!("{}", cnt);
}
