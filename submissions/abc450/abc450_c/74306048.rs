use std::collections::BTreeMap;

use ac_library::Dsu;
use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    h: usize,
    w: usize,
    s: [Chars; h],
  }

  let dots: BTreeMap<_, usize> = s.iter().enumerate().map(|(y, row)| row.iter().enumerate().filter(|&(_, &i)| i == '.').map(move |(x, _)| (x, y))).flatten().zip(1..).collect();
  let mut g = Dsu::new(dots.len() + 1);

  eprintln!("{:?}", dots);

  for (&(x, y), &i) in dots.iter() {
    let mut a = i;
    eprintln!("{} {}", x, y);
    if x == 0 || y == 0 || x == h - 1 || y == w - 1 {
      a = 0;
    }
    if y >= 1 && s[y - 1][x] == '.' {
      let &b = dots.get(&(x, y - 1)).unwrap();
      g.merge(a, b);
    }
    if x >= 1 && s[y][x - 1] == '.' {
      let &b = dots.get(&(x - 1, y)).unwrap();
      g.merge(a, b);
    }
    if x < w - 1 && s[y][x + 1] == '.' {
      let &b = dots.get(&(x + 1, y)).unwrap();
      g.merge(a, b);
    }
    if y < h - 1 && s[y + 1][x] == '.' {
      let &b = dots.get(&(x, y + 1)).unwrap();
      g.merge(a, b);
    }
  }

  println!("{}", g.groups().iter().filter(|&edges| !edges.into_iter().any(|&e| e == 0)).count());
}
