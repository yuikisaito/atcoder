use indexmap::IndexMap;
use proconio::input;

fn main() {
  input! {
    n: usize,
    s: [String; n],
  }

  let mut map = IndexMap::new();
  let mut max = 0;
  for i in s {
    let n = map.entry(i.to_lowercase()).and_modify(|e| *e += 1).or_insert(1);
    max = max.max(*n);
  }

  println!("{}", max);
}
