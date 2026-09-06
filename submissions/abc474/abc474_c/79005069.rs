use indexmap::IndexSet;
use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    n: usize,
    q: usize,
    p: [usize; n],
    a: [usize; q],
  }

  let mut set = IndexSet::new();
  for i in a.into_iter().rev() {
    set.insert(i);
  }
  for i in p.into_iter().rev() {
    set.insert(i);
  }
  set.reverse();

  println!("{}", set.into_iter().join(" "));
}
