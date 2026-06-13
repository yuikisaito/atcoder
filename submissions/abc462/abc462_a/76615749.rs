use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    s: Chars,
  }

  println!("{}", s.into_iter().filter(|c| matches!(c, '0'..='9')).collect::<String>());
}
