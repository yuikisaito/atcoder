use proconio::input;

fn main() {
  input! {
    c: char,
  }

  println!(
    "{}",
    if c == 'B' {
      'Y'
    } else if c == 'Y' {
      'R'
    } else {
      'B'
    }
  );
}
