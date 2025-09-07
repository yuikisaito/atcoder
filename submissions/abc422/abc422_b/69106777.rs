use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    h: usize,
    w: usize,
    s: [Chars; h]
  }

  for x in 0..w {
    for y in 0..h {
      if s[y][x] == '#' {
        let mut cnt = 0;
        if y > 0 && s[y - 1][x] == '#' {
          cnt += 1;
        }
        if x > 0 && s[y][x - 1] == '#' {
          cnt += 1;
        }
        if y < h - 1 && s[y + 1][x] == '#' {
          cnt += 1;
        }
        if x < w - 1 && s[y][x + 1] == '#' {
          cnt += 1;
        }
        if cnt == 2 || cnt == 4 {
          continue;
        } else {
          println!("No");
          return
        }
      }
    }
  }

  println!("Yes");
}
