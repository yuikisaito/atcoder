use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    t: usize,
    cases: [(usize, Chars); t],
  }

  for (n, s) in cases {
    let split_point = n / 2;
    let left = &s[..split_point];
    let right = &s[split_point..];

    println!("left: {:?}", left);
    println!("right: {:?}", right);

    let pattern_char = ['0', '1'];
    let pattern_rev_char = ['1', '0'];

    let mut counts = Vec::new();

    for pattern_index in 0..=1 {
      let pattern = pattern_char[pattern_index];
      let pattern_rev = pattern_rev_char[pattern_index];

      let right_limit = left.iter().rposition(|&i| i == pattern_rev);
      let left_limit = right.iter().position(|&i| i == pattern_rev);

      let left_moves = if let Some(limit) = right_limit {
        left
          .iter()
          .enumerate()
          .filter(|&(i, &c)| i <= limit && c == pattern)
          .count()
      } else {
        0
      };
      let right_moves = if let Some(limit) = left_limit {
        right
          .iter()
          .enumerate()
          .filter(|&(i, &c)| limit <= i && c == pattern)
          .count()
      } else {
        0
      };
      let inverts = s.iter().filter(|&&c| c == pattern_rev).count();
      counts.push((left_moves + right_moves) * 2 + inverts);
    }
    println!("{}", counts.iter().min().unwrap());
  }
}
