use proconio::input;

fn main() {
  input! {
    s: usize,
    a: usize,
    b: usize,
    x: usize,
  }

  let interval = a + b;
  println!("{}", x / interval * s * a + a.min(x % interval) * s);
}
