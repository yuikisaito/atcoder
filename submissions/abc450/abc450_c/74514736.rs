use std::collections::HashSet;

use ac_library::Dsu;
use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
      h: usize,
      w: usize,
      s: [Chars; h],
  }

  let mut dsu = Dsu::new(h * w);

  for i in 0..h {
    for j in 0..w - 1 {
      if s[i][j] == '.' && s[i][j + 1] == '.' {
        dsu.merge(i * w + j, i * w + j + 1);
      }
    }
  }

  for i in 0..h - 1 {
    for j in 0..w {
      if s[i][j] == '.' && s[i + 1][j] == '.' {
        dsu.merge(i * w + j, (i + 1) * w + j);
      }
    }
  }

  let mut all_roots: HashSet<usize> = HashSet::new();
  let mut border_roots: HashSet<usize> = HashSet::new();

  for i in 0..h {
    for j in 0..w {
      if s[i][j] == '.' {
        let root = dsu.leader(i * w + j);
        all_roots.insert(root);

        if i == 0 || i == h - 1 || j == 0 || j == w - 1 {
          border_roots.insert(root);
        }
      }
    }
  }

  let ans = all_roots.len() - border_roots.len();
  println!("{}", ans);
}
