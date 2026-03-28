use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    q: usize,
    queries: [(usize, usize); q],
  }

  let mut trees = Vec::new();
  for (r#type, h) in queries.into_iter() {
    match r#type {
      1 => trees.push(h),
      2 => {
        trees = trees.into_iter().filter(|&t| t > h).collect_vec();
      },
      _ => unreachable!(),
    }
    println!("{}", trees.len());
  }
}
