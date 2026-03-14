use proconio::input;

fn main() {
  input! {
    mut h: usize,
    mut w: usize,
    q: usize,
    queries: [(usize, usize); q],
  }

  for (r#type, eat) in queries {
    match r#type {
      1 => {
        println!("{}", w * eat);
        h -= eat;
      },
      2 => {
        println!("{}", h * eat);
        w -= eat;
      },
      _ => unreachable!(),
    }
  }
}
