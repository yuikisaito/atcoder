use proconio::input;

fn main() {
  input! {
    n: usize,
    pab: [(usize, usize, usize); n],
    q: usize,
    x: [usize; q],
  }

  for xi in x {
    let mut tension = xi;
    for &(p, a, b) in pab.iter() {
      if p >= tension {
        tension += a;
      } else if tension <= b {
        tension = 0;
      } else {
        tension -= b;
      }
    }
    println!("{}", tension)
  }
}
