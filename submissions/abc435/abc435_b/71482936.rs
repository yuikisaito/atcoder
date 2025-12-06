use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [usize; n],
  }

  let mut cnt = 0;
  for c in (1..=n).combinations(2) {
    let l = c[0];
    let r = c[1];
    let sum = a[l - 1..r].iter().sum::<usize>();
    if a[l - 1..r].iter().all(|&i| sum % i != 0) {
      cnt += 1;
    }
  }

  println!("{}", cnt);
}
