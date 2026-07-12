use proconio::input;

fn main() {
  input! {
      n: usize,
      k: usize,
      ab: [[isize; 2]; n],
  }

  let mut dp = vec![isize::MIN; 2 * k + 1];
  dp[0] = 0;

  for t in ab {
    for j in (0..=2 * k).rev() {
      if dp[j] == isize::MIN {
        continue;
      }

      // 切り替える
      if j < 2 * k {
        dp[j + 1] = dp[j + 1].max(dp[j] + t[1 - (j & 1)]);
      }

      // 切り替えない
      dp[j] = dp[j] + t[j & 1];
    }
  }

  println!("{}", dp.iter().max().unwrap());
}
