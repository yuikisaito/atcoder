use proconio::input;
use proconio::marker::Bytes;

fn f(
  limit: isize,
  s: &[Vec<u8>],
  h: usize,
  w: usize,
) -> isize {
  if limit < 0 {
    return 0;
  }

  let mut res = 0;

  for t in 0..h {
    let mut col = vec![0; w];

    for b in t..h {
      for j in 0..w {
        col[j] += (s[b][j] - b'0') as isize;
      }

      let mut l = 0;

      let mut sum = 0;

      for r in 0..w {
        sum += col[r];

        while sum > limit {
          sum -= col[l];

          l += 1;
        }

        res += r - l + 1;
      }
    }
  }

  res as isize
}

fn main() {
  input! {
      h: usize,
      w: usize,
      k: isize,
      s: [Bytes; h],
  }

  let ans = f(k, &s, h, w) - f(k - 1, &s, h, w);

  println!("{}", ans);
}
