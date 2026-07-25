use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    n: usize,
    p: [usize; n],
    q: [usize; n],
  }

  let get_index = |x: Vec<usize>| {
    return (1..=n).permutations(n).position(|y| x == y).unwrap();
  };

  println!("{}", get_index(q).saturating_sub(get_index(p) + 1));
}
