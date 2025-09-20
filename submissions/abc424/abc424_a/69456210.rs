use proconio::input;

fn main() {
  input! {
    a: usize,
    b: usize,
    c: usize,
  }

  println!(
    "{}",
    if a == b || b == c || c == a {
      "Yes"
    } else {
      "No"
    }
  );
}
