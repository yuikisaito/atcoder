use itertools::izip;
use proconio::input;

fn main() {
  input! {
    n: usize,
    _m: usize,
    a: [usize; n],
    b: [usize; n - 1],
  }

  let mut ans = 0;
  let mut rev = false;
  for (x, y) in izip!(a.windows(2), b.into_iter()) {
    let sum: usize = x.iter().sum();
    if (sum + rev as usize) % 2 != y {
      rev = true;
      ans += 1;
    } else {
      rev = false;
    }
  }

  println!("{}", ans);
}
