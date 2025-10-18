use std::collections::HashSet;

use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    n: usize,
    k: usize,
    s: Chars,
  }

  let mut words = Vec::new();
  for i in 0..n - k + 1 {
    words.push(s.iter().skip(i).take(k).collect::<String>());
  }

  let indexed_words = words.iter().map(|word| {
    let rtn = words.iter().filter(move |&w| w == word);
    (rtn.clone().next().unwrap(), rtn.count())
  });
  let max_count = indexed_words.clone().map(|(_, y)| y).max().unwrap();
  let max_words = indexed_words
    .filter(|&(_, y)| y == max_count)
    .map(|(x, _)| x.clone())
    .collect::<HashSet<String>>();

  println!("{}", max_count);
  println!("{}", max_words.iter().join(" "));
}
