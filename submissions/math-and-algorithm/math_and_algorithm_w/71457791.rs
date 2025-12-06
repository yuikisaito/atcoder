use proconio::input;

fn main() {
  input! {
    n: usize,
    b: [usize; n],
    r: [usize; n],
  }

  println!(
    "{:.6}",
    (b.into_iter().sum::<usize>() + r.into_iter().sum::<usize>()) as f64 / n as f64
  );
}
