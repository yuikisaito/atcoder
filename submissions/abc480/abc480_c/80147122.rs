use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    n: usize,
    k: usize,
    t: usize,
    a: [usize; n],
  }

  let mut now = 0;
  let mut slot = a.into_iter().peekable();
  let mut manual = 0;
  while now < t {
    if slot.peek().unwrap_or(&usize::MAX) <= &now {
      now = slot.next().unwrap() + k;
      continue;
    }
    manual += 1;
    now += k;
  }

  println!("{}", manual);
}
