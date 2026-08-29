use indexmap::IndexSet;
use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [usize; n],
  }

  let mut ans = IndexSet::new();

  for k in a {
    if !ans.insert(k) {
      ans.swap_remove(&k);
    }
  }

  println!("{}", ans.into_iter().sum::<usize>());
}
