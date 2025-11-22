use proconio::input;

fn main() {
  input! {
    x: usize,
    y: usize,
    z: usize,
  }

  let mut year = 0;
  loop {
    let rate = (x + year) as f64 / (y + year) as f64;
    if y + year == 0 {
    } else if rate == z as f64 {
      println!("Yes");
      return
    } else if rate < z as f64 {
      break;
    }
    year += 1;
  }
  println!("No")
}
