use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    s: Chars,
  }

  let t_pos = s
    .iter()
    .enumerate()
    .filter(|(_, &c)| c == 't')
    .map(|(i, _)| i);

  let split_points = t_pos.combinations(2);

  let mut result = 0.0;
  for p in split_points {
    let target = &s[p[0] + 1..p[1]];
    result = f64::max(
      result,
      target.iter().filter(|&&c| c == 't').count() as f64 / target.len() as f64,
    );
  }
  println!("{:.10}", result);
}
