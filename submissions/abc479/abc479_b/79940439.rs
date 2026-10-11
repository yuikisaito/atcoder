use std::ops::Deref;
use std::ops::DerefMut;

use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    _: usize,
    s: Chars,
  }

  println!(
    "{}",
    s.into_iter()
      .scan((0, 'R'), |state, c| {
        if c == 'R' {
          state.0 += 1;
        } else {
          state.0 = 0;
        }
        state.1 = c;
        Some(*state)
      })
      .map(|(cnt, _)| cnt)
      .max()
      .unwrap()
  );
}
