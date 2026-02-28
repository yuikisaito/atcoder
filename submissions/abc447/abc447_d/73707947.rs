use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    s: Chars,
  }

  let mut available_a = 0;
  let mut available_b = 0;
  let mut available_c = 0;
  for c in s {
    if c == 'A' {
      available_a += 1;
    } else if c == 'B' && available_a > 0 {
      available_a -= 1;
      available_b += 1;
    } else if c == 'C' && available_b > 0 {
      available_b -= 1;
      available_c += 1;
    }
  }

  println!("{}", available_c);
}
