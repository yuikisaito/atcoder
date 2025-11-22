use proconio::input;

fn main() {
  input! {
    x: usize,
    y: usize,
    z: usize,
  }

  for year in 0..=100 - x.max(y) {
    if x + year == (y + year) * z {
      println!("Yes");
      return
    }
  }
  println!("No")
}
