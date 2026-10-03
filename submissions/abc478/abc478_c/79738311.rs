use itertools::Itertools;
use itertools::izip;
use proconio::input;

fn main() {
  input! {
    n: usize,
    k: usize,
    a: [usize; n],
  }

  let sorted = a.clone().into_iter().sorted().collect_vec();
  let mut start = n;
  let mut end = n;
  for (i, x, y) in izip!(0.., a, sorted) {
    if x != y && i < start {
      start = i;
    }
    if start < i && x != y {
      end = i;
    }
  }

  println!("{}", if end - start + 1 <= k { "Yes" } else { "No" });
}
