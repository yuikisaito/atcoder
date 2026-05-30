use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    n: usize,
    m: usize,
    a: [usize; n],
    b: [usize; m],
  }

  let sharis = a.into_iter().sorted().rev();
  let mut netas = b.into_iter().sorted().rev();

  let mut cnt = 0;
  'outer: for shari in sharis {
    loop {
      if let Some(neta) = netas.next() {
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
