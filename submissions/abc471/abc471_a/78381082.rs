use proconio::input;

fn main() {
  input! {
    a: usize,
    b: usize,
  }

  println!("{}", if [a + b, a.saturating_sub(b), a * b].contains(&9) || a as f64 / b as f64 == 9.0 { "Nine" } else { "Nein" });
}
