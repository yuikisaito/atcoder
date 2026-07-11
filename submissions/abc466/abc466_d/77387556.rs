use proconio::input;
use proconio::marker::Usize1;

fn main() {
  input! {
    n: usize,
    m: usize,
    rc: [(Usize1, Usize1); m],
  }

  let mut rows = vec![false; n];
  let mut cols = vec![false; n];

  let mut ans = 0;
  for (r, c) in rc.into_iter().rev() {
    if !rows[r] && !cols[c] {
      rows[r] = true;
      cols[c] = true;

      ans += 1;
    }
  }

  println!("{}", ans);
}
