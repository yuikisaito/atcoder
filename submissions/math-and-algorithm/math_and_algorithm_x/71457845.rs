use proconio::input;

fn main() {
  input! {
    n: usize,
    pq: [(usize, usize); n],
  }

  println!(
    "{:.6}",
    pq.into_iter()
      .map(|(p, q)| q as f64 / p as f64)
      .sum::<f64>()
  );
}
