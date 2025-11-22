use proconio::input;

fn main() {
  input! {
    x: usize,
    y: usize,
    z: usize,
  }

  let mut year = 0;
  loop {
    if y + year == 0 {
    } else if (x + year) / (y + year) == z {
      println!("Yes");
      return
    } else if (x + year) / (y + year) < z {
      break;
    }
    year += 1;
  }
  println!("No")
}
