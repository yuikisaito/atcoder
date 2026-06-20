use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    n: usize,
    x: char,
    s: [Chars; n],
  }

  let col = x as u8 - b'A';

  println!("{}", if s.into_iter().any(|row| row[col as usize] == 'o') { "Yes" } else { "No" });
}
