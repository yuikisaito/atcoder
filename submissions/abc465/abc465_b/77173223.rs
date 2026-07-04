use proconio::input;

fn main() {
  input! {
    x: isize,
    y: isize,
    l: isize,
    r: isize,
    a: isize,
    b: isize,
  }

  eprintln!("{}", (l..r).filter(|&t| a <= t && t < b).count() as isize);
  println!("{}", (b - a) * y + (x - y) * (l..r).filter(|&t| a <= t && t < b).count() as isize);
}
