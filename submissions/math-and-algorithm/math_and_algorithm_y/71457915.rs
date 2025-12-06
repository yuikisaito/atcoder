use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [usize; n],
    b: [usize; n],
  }

  println!(
    "{:.3}",
    a.into_iter()
      .zip(b)
      .map(|(x, y)| x as f64 / 3. + y as f64 * 2. / 3.)
      .sum::<f64>()
  );
}
