use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    n: usize,
    m: usize,
    s: Chars,
    t: Chars,
  }

  println!(
    "{}",
    (0..n - m + 1)
      .map(|i| (i..i + m)
        .zip(0..m)
        .map(|(j, k)| (10 + s[j].to_digit(10).unwrap() - t[k].to_digit(10).unwrap()) % 10)
        .sum::<u32>())
      .min()
      .unwrap()
  );
}
