use proconio::input;

fn main() {
  input! {
    h: usize,
    w: usize,
  }

  for y in 0..h {
    for x in 0..w {
      print!("{}", if [0, h - 1].contains(&y) || [0, w - 1].contains(&x) { "#" } else { "." })
    }
    print!("\n");
  }
}
