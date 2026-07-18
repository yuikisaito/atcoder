use itertools::izip;
use proconio::input;

fn main() {
  input! {
    n: usize,
    _m: usize,
    a: [usize; n],
    b: [usize; n - 1],
  }

  println!(
    "{}",
    [true, false]
      .into_iter()
      .map(|mut rev| {
        let mut ans = 0;
        for (x, y) in izip!(a.windows(2), b.iter()) {
          let sum: usize = x.iter().sum();
          if (sum + rev as usize) % 2 != *y {
            rev = true;
            ans += 1;
          } else {
            rev = false;
          }
        }

        ans
      })
      .min()
      .unwrap()
  );
}
