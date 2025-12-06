use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [usize; n],
  }

  let mut taoreru = 0;

  for (&h, crnt) in a.iter().zip(0..n) {
    if crnt + h >= n {
      break;
    } else if crnt <= taoreru {
      if crnt + h - 1 > taoreru {
        taoreru = crnt + h - 1;
      }
    } else {
      println!("{}", crnt);
      return;
    }
  }

  println!("{}", n);
}
