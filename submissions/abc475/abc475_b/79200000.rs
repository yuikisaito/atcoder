use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [usize; n],
  }

  let mut coins = [0; 3];
  a.into_iter().for_each(|i| {
    let mut remaining = ((i as f64 / 1000.).ceil() * 1000.) as usize - i;
    coins.iter_mut().for_each(|i| {
      *i += remaining % 10;
      remaining /= 10;
    });
  });

  println!("{}", coins.into_iter().join(" "));
}
