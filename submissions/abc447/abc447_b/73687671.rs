use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    s: Chars,
  }

  let appear = s.clone().into_iter().counts();
  let max = appear.iter().map(&|(_, &n)| n).max().unwrap();

  let banned = appear.into_iter().filter(|&(_, n)| n == max).map(|(c, _)| c).collect_vec();

  println!("{}", s.into_iter().filter(|c| !banned.contains(&c)).join(""));
}
