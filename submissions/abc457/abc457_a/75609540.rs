use proconio::input;
use proconio::marker::Usize1;

fn main() {
  input! {
    n: usize,
    a: [usize; n],
    x: Usize1,
  }

  println!("{}", a[x]);
}
