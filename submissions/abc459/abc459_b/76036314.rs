use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    n: usize,
    s: [Chars; n],
  }

  println!(
    "{}",
    s.into_iter()
      .map(|str| match str[0] {
        'a'..='c' => 2,
        'd'..='f' => 3,
        'g'..='i' => 4,
        'j'..='l' => 5,
        'm'..='o' => 6,
        'p'..='s' => 7,
        't'..='v' => 8,
        'w'..='z' => 9,
        _ => unreachable!(),
      })
      .join("")
  );
}
