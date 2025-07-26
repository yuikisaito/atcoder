use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    _: usize,
    l: usize,
    r: usize,
    s: Chars,
  }

  if s[(l - 1)..r].into_iter().all(|&si| si == 'o') {
    println!("Yes")
  } else {
    println!("No")
  }
}
