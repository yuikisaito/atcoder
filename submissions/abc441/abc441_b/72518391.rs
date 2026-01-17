use std::collections::HashSet;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
  input! {
    _n: usize,
    _m: usize,
    s: Chars,
    t: Chars,
    q: usize,
    w: [Chars; q],
  }

  let all_chars = s.iter().chain(t.iter()).collect::<HashSet<_>>();
  let only_taka = all_chars.iter().filter(|&c| !t.contains(c)).collect_vec();
  let only_ao = all_chars.iter().filter(|&c| !s.contains(c)).collect_vec();

  for word in w {
    println!(
      "{}",
      if word.iter().any(|&c| only_taka.contains(&&&c)) {
        "Takahashi"
      } else if word.iter().any(|&c| only_ao.contains(&&&c)) {
        "Aoki"
      } else {
        "Unknown"
      }
    );
  }
}
