use std::collections::HashSet;

use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    n: usize,
    m: usize,
    s: [Chars; n],
  }

  let mut hs = HashSet::<String>::new();

  for ys in 0..=(n - m) {
    let yl = ys + m;
    for xs in 0..=(n - m) {
      let xl = xs + m;
      let mut chars = Vec::new();
      for yi in ys..yl {
        chars.push(s[yi][xs..xl].to_vec());
      }
      hs.insert(chars.iter().flatten().collect());
    }
  }

  println!("{}", hs.len());
}
