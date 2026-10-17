use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    n: usize,
    k: usize,
    t: usize,
    a: [usize; n],
  }

  let mut time = 0..t;
  let mut slot = a.into_iter().peekable();
  let mut manual = 0;
  while let Some(now) = time.next() {
    if slot.peek().unwrap_or(&usize::MAX) <= &now {
      time.by_ref().take(slot.next().unwrap() + k - now - 1).collect_vec();
      continue;
    }
    manual += 1;
    time.by_ref().take(k - 1).collect_vec();
  }

  println!("{}", manual);
}
