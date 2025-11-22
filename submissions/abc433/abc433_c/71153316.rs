use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    s: Chars,
  }

  let mut zipped = Vec::new();
  let mut last = s[0];
  let mut crnt = 0;
  for si in s {
    if last != si {
      zipped.push((last as usize, crnt));
      crnt = 0;
      last = si;
    }
    crnt += 1;
  }
  zipped.push((last as usize, crnt));

  let mut cnt = 0;
  for w in zipped.windows(2) {
    if w[0].0 + 1 == w[1].0 {
      cnt += w[0].1.min(w[1].1);
    }
  }

  println!("{}", cnt);
}
