use itertools::Itertools;
use text_io::read;

fn main() {
  let n: usize = read!();
  let q: usize = read!();
  let mut a = Vec::new();
  for _ in 0..n {
    let i: usize = read!();
    a.push(i);
  }

  let min = (0..).zip(a.into_iter()).sorted_by_key(|x| x.1).collect_vec();

  for _ in 0..q {
    let k: usize = read!();
    let mut b = Vec::new();
    for _ in 0..k {
      let i: usize = read!();
      b.push(i - 1);
    }
    for &(i, num) in min.iter() {
      if !b.contains(&i) {
        println!("{}", num);
        break;
      }
    }
  }
}
