use text_io::read;

fn main() {
  let n: usize = read!();
  let q: usize = read!();
  let mut a = Vec::<usize>::new();
  for _ in 0..n {
    a.push(read!());
  }

  for _ in 0..q {
    let t: usize = read!();
    match t {
      1 => {
        let x: usize = read!();
        a.swap(x - 1, x);
      },
      2 => {
        let l: usize = read!();
        let r: usize = read!();
        println!("{}", a[l - 1..=r - 1].iter().sum::<usize>())
      },
      _ => unreachable!(),
    }
  }
}
