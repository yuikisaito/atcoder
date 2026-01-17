use proconio::input;

fn main() {
  input! {
    p: usize,
    q: usize,
    x: usize,
    y: usize,
  }

  println!(
    "{}",
    if (p..p + 100).contains(&x) && (q..q + 100).contains(&y) {
      "Yes"
    } else {
      "No"
    }
  );
}
