use num::ToPrimitive;
use text_io::read;

fn main() {
  let n: usize = read!();
  let q: usize = read!();

  let mut a: Vec<usize> = vec![];
  for _ in 0..n {
    a.push(read!());
  }

  let mut pos = 0;
  let total = a.iter().sum::<usize>();

  for _ in 0..q {
    let t: usize = read!();
    match t {
      1 => {
        let c: usize = read!();
        pos = (pos + c) % n;
      },
      2 => {
        let l: usize = read!();
        let r: usize = read!();
        let l_pos: usize = ((pos as isize + l as isize % n as isize + n as isize - 1) % n as isize)
          .to_usize()
          .unwrap();
        let r_pos = ((pos as isize + r as isize % n as isize + n as isize - 1) % n as isize)
          .to_usize()
          .unwrap();
        let sum: usize;
        if l_pos <= r_pos {
          sum = a[l_pos..=r_pos].iter().sum();
        } else {
          sum = total - a[r_pos + 1..l_pos].iter().sum::<usize>();
        }
        println!("{}", sum);
      },
      _ => unreachable!(),
    }
  }
}
