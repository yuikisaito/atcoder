use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    n: usize,
    s: Chars,
  }

  let a_pos = s
    .iter()
    .enumerate()
    .filter(|(_, &c)| c == 'A')
    .map(|(i, _)| i);
  let result: usize = (0..2 * n)
    .filter(|&n| n as f32 % 2.0 == 0.0)
    .zip(a_pos.clone())
    .map(|(a, b)| a.max(b) - a.min(b))
    .sum::<usize>()
    .min(
      (0..2 * n)
        .filter(|&n| n as f32 % 2.0 != 0.0)
        .zip(a_pos)
        .map(|(a, b)| a.max(b) - a.min(b))
        .sum::<usize>(),
    );

  println!("{}", result);
}
