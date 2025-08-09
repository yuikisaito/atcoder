use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    _: usize,
    s: Chars,
  }

  if s.iter().collect::<String>().ends_with("tea") {
    println!("Yes");
  } else {
    println!("No");
  }
}
