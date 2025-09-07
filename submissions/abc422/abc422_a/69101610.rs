use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    s: Chars,
  }

  let i = s[0].to_digit(10).unwrap();
  let j = s[2].to_digit(10).unwrap();

  let next_i = if j < 8 { i } else { i + 1 };
  let next_j = if j < 8 { j + 1 } else { 1 };

  println!("{}-{}", next_i, next_j);
}
