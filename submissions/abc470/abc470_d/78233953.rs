use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
  input! {
    n: usize,
    q: usize,
    p: [Usize1; n],
  }

  let mut pq = vec![p, vec![0; n]];
  for x in 0..n {
    let y = pq[0][x];
    pq[1][y] = x;
  }

  let mut pi = 0;

  for _ in 0..q {
    input! {
      t: usize
    }

    if t == 1 {
      input! {
        x: Usize1,
        y: Usize1,
      }

      let px = pq[pi][x];
      let py = pq[pi][y];

      pq[pi].swap(x, y);
      pq[pi ^ 1].swap(px, py);
    } else {
      pi ^= 1;
    }
  }

  println!("{}", pq[pi].iter().map(|x| x + 1).join(" "));
}
