use proconio::input;

fn main() {
  input! {
    a: usize,
    b: usize,
    c: usize,
    d: usize,
  }

  if a <= c && d < b {
    println!("Yes")
  } else {
    println!("No")
  }
}
