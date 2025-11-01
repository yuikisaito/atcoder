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
    let r_iter = ((l + 1.max(a) - 1)..n).rev();
    for r in r_iter.clone() {
      if a <= s[l..=r].iter().filter(|&&c| c == 'a').count()
        && s[l..=r].iter().filter(|&&c| c == 'b').count() < b
      {
        cnt += r_iter.count() - (n - r);
        break;
      }
    }
  }

  println!("{}", cnt);
}
