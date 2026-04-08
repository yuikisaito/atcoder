use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    s: Chars,
    t: Chars,
  }

  let mut dp = vec![0; t.len() + 1];
  let mut ans = 0;

  for c in s {
    dp[0] += 1;

    for i in (0..t.len()).rev() {
      if t[i] == c {
        dp[i + 1] += dp[i];
        dp[i] = 0;
      }
    }

    ans += dp.iter().rev().skip(1).sum::<usize>();
  }

  println!("{}", ans);
}
