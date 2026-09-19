use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    mut s: Chars,
  }

  if s.last().unwrap() != &'e' {
    s.push('e');
  }
  s.push('r');

  println!("{}", s.into_iter().collect::<String>());
}
