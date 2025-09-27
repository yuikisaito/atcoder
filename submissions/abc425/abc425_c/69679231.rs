use text_io::read;

fn main() {
  let n: usize = read!();
  let q: usize = read!();

  let mut a: Vec<usize> = vec![];
  for _ in 0..n {
    a.push(read!());
  }

  let mut pos = 0;

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
        let l_pos = (pos + l - 1) % n;
        let r_pos = (pos + r - 1) % n;
        let sum: usize;
        if l_pos <= r_pos {
          sum = a[l_pos..=r_pos].iter().sum();
        } else {
          let front = a.iter().take(r_pos + 1).sum::<usize>();
          let back = a.iter().rev().take(n - l_pos).sum::<usize>();
          sum = front + back;
        }
        println!("{}", sum);
      },
      _ => unreachable!(),
    }
  }
}
