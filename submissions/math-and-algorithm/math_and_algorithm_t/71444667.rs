use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [usize; n],
  }

  let mut dp = vec![vec![0; 1001]; 6];
  dp[0][0] = 1;

  for x in a {
    for i in (1..=5).rev() {
      for j in (x..=1000).rev() {
        dp[i][j] += dp[i - 1][j - x]
      }
    }
  }

  println!("{}", dp[5][1000]);
}
