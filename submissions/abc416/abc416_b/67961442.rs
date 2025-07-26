use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    s: Chars,
  }

  let s_len = s.len();
  let mut result = vec!['.'; s_len];
  let hash_pos = s.into_iter().positions(|c| c == '#').collect_vec();

  for i in hash_pos.clone() {
    result[i] = '#';
  }

  if hash_pos.len() == 0 || *hash_pos.first().unwrap() > 0 {
    result[0] = 'o';
  }
  for pos in hash_pos.windows(2) {
    let left = pos[0];
    let right = pos[1];
    if right - left > 1 {
      result[left + 1] = 'o';
    }
  }

  if *hash_pos.last().unwrap() < s_len - 1 {
    result[s_len - 1] = 'o';
  }

  println!("{}", result.iter().collect::<String>());
}
