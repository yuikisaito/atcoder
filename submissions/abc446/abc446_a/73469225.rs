use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    s: Chars,
  }

  println!("Of{}", s.into_iter().map(|c| c.to_ascii_lowercase()).collect::<String>());
}
