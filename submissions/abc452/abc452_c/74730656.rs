use itertools::izip;
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    n: usize,
    ab: [(usize, usize); n],
    m: usize,
    s: [Chars; m],
  }

  let (a, b): (Vec<_>, Vec<_>) = ab.into_iter().unzip();

  let mut memo = (1..=10).map(|i| (0..i).map(|_| vec![false; 26]).collect_vec()).collect_vec();

  for str in s.iter() {
    for (&c, i) in str.iter().zip(0..) {
      memo[str.len() - 1][i][(c as u8 - b'a') as usize] = true;
    }
  }

  println!(
    "{}",
    s.into_iter().map(|str| if str.len() == n && izip!(a.iter(), b.iter(), str).all(|(&len, &i, c)| memo[len - 1][i - 1][(c as u8 - b'a') as usize]) { "Yes" } else { "No" }).join("\n")
  );
}
