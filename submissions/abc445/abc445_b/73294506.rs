use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
  input! {
    n: usize,
    s: [Chars; n],
  }

  let len = s.iter().map(|s| s.len()).collect_vec();
  let m = len.iter().max().unwrap();

  for i in 0..n {
    let dots = ".".repeat((m-len[i])/2);
    println!("{}{}{}", dots, s[i].iter().collect::<String>(), dots);
  }
}
