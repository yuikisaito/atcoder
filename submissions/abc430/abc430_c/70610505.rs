use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    n: usize,
    a: usize,
    b: usize,
    s: Chars,
  }

  let mut cnt = 0;

  for l in 0..(n - a + 1) {
    for r in l..n {
      if a <= s[l..=r].iter().filter(|&&c| c == 'a').count()
        && s[l..=r].iter().filter(|&&c| c == 'b').count() < b
      {
        cnt += 1;
      }
    }
  }

  println!("{}", cnt);
}
