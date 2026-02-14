use proconio::{input, marker::Chars};

fn main() {
  input! {
    s: Chars,
  }

  println!("{}", if s.first().unwrap() == s.last().unwrap() {
    "Yes"
  } else {
    "No"
  });
}
