use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    _: usize,
    l: usize,
    r: usize,
    s: Chars,
  }

  if s.get(l - 1..r).into_iter().all(|si| si[0] == 'o') {
    println!("Yes")
  } else {
    println!("No")
  }
}
