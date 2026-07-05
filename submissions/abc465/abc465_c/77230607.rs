use std::collections::VecDeque;

use itertools::izip;
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    _n: usize,
    s: Chars,
  }

  let mut ans = VecDeque::new();

  let mut rev = false;

  for (c, n) in s.into_iter().zip(1..) {
    if rev {
      ans.push_front(n);
    } else {
      ans.push_back(n);
    }
    rev ^= c == 'o';
  }

  if rev {
    ans.make_contiguous().reverse();
  }

  println!("{}", ans.into_iter().map(|n| n.to_string()).join(" "));
}
