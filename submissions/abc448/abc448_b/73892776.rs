use proconio::input;

fn main() {
  input! {
    n: usize,
    m: usize,
    c: [usize; m],
    ab: [(usize, usize); n],
  }

  let mut max = vec![0; m];
  for (a, b) in ab {
    max[a - 1] += b;
  }

  println!("{}", max.into_iter().zip(c.into_iter()).map(|(max, exist)| max.min(exist)).sum::<usize>());
}
