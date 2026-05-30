use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    n: usize,
    m: usize,
    a: [usize; n],
    b: [usize; m],
  }

  let mut sharis = a.into_iter().sorted();
  let netas = b.into_iter().sorted();

  let mut cnt = 0;
  'outer: for neta in netas {
    loop {
      if let Some(shari) = sharis.next() {
        if neta <= shari * 2 {
          cnt += 1;
          continue 'outer
        }
      } else {
        break 'outer
      }
    }
  }

  println!("{}", cnt);
}
