use proconio::input;

fn main() {
  input! {
    n: usize,
    mut x: usize,
    s: [usize; n],
  }

  for i in s {
    println!(
      "{}",
      if i < x {
        x = i;
        "1"
      } else {
        "0"
      }
    )
  }
}
