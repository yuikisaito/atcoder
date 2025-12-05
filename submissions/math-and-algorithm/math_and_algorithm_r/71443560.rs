use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [usize; n],
  }

  println!(
    "{}",
    a.iter().filter(|&&i| i == 100).count() * a.iter().filter(|&&i| i == 400).count()
      + a.iter().filter(|&&i| i == 200).count() * a.iter().filter(|&&i| i == 300).count()
  );
}
