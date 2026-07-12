use proconio::input;

fn main() {
  input! {
    n: usize,
    k: usize,
    ab: [[isize; 2]; n],
  }

  let mut dp = vec![vec![isize::MIN; 2 * k + 1]; n + 1];
  dp[0][0] = 0;

  for (t, i) in ab.into_iter().zip(1..) {
    for j in (0..=2 * k).rev() {
      if dp[i - 1][j] == isize::MIN {
        continue;
      }

      // 切り替えない（切り替え後にも発生する）
      dp[i][j] = dp[i - 1][j] + t[j & 1];

      // 切り替える（2*k回しか発生しない）
      if j < 2 * k {
        dp[i][j + 1] = dp[i][j + 1].max(dp[i - 1][j] + t[1 - (j & 1)]);
      }
    }
  }

  println!("{}", dp[n].iter().max().unwrap());
}
