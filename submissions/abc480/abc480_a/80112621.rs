use proconio::input;
use proconio::marker::Chars;

fn main() {
  let mut a = (0usize, 0usize);
  let mut d = (0usize, 0usize);
  for y in 0..2 {
    input! {
      row: Chars,
    }

    for x in 0..2 {
      match row[x] {
        'A' => {
          a = (x, y);
        },
        'D' => {
          d = (x, y);
        },
        _ => {},
      }
    }
  }

  println!("{}", if a.0.abs_diff(d.0) + a.1.abs_diff(d.1) <= 1 { "Yes" } else { "No" });
}
