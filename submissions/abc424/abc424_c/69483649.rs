use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    n: usize,
    ab: [(usize, usize); n]
  }

  let mut skills = vec![false; n];
  let sorted = ab
    .iter()
    .enumerate()
    .sorted_by(|&(_, &a), &(_, &b)| Ord::cmp(&a.0.min(a.1), &b.0.min(b.1)));
  for (i, &(a, b)) in sorted {
    if (a == 0 && b == 0) || skills[a - 1] || skills[b - 1] {
      skills[i] = true;
    }
  }

  println!("{}", skills.iter().filter(|&&i| i).count());
}
