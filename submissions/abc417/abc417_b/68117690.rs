use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    n: usize,
    m: usize,
    mut a: [usize; n],
    b: [usize; m],
  }

  for bi in b {
    if let Some(pos) = a.iter().position(|&x| x == bi) {
      a.remove(pos);
    }
  }

  println!(
    "{}",
    a.iter().map(|i| i.to_string()).collect_vec().join(" ")
  );
}
