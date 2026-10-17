use indexmap::IndexMap;
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    n: usize,
    m: usize,
    s: [Chars; n],
    t: [Chars; m],
  }

  let mut candidates = IndexMap::<_, _>::from_iter(s.iter().cloned().dedup().enumerate().map(|(k, v)| (v, k)));
  for x in 0..10 {
    for (i, mut c) in s.iter().cloned().enumerate() {
      if x < c.len() {
        c.remove(x);
        if candidates.swap_remove(&c) == None {
          candidates.insert(c, i);
        }
      }
    }
  }

  let mut votes = vec![0; n];

  for v in t {
    if let Some(c) = candidates.get(&v) {
      votes[*c] += 1;
      continue;
    }
    let a = (0..v.len())
      .filter_map(|x| {
        let mut v = v.clone();
        v.remove(x);
        candidates.get(&v)
      })
      .collect_vec();
    if a.len() == 1 {
      votes[*a[0]] += 1;
    }
  }

  println!("{}", votes.into_iter().join("\n"));
}
