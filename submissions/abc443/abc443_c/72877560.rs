use std::usize;

use proconio::input;

fn main() {
  input! {
    n: usize,
    t: usize,
    a: [usize; n],
  }

  let mut open = 0;
  let mut secs = 0;
  for i in a {
    if i >= open {
      secs += i - open;
      open = i + 100;
    }
  }
  secs += t.saturating_sub(open);

  println!("{}", secs);
}
