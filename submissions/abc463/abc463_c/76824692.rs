use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    n: usize,
    hl: [(usize, usize); n],
    q: usize,
    t: [usize; q],
  }

  let mut highest = 0;
  let mut height = Vec::new();
  let mut time = Vec::new();
  for (h, l) in hl.into_iter().sorted_by_key(|&(_, l)| -(l as isize)) {
    if h > highest {
      height.push(h);
      time.push(l);
      highest = h;
    }
  }

  for a in t {
    let i = time.iter().take_while(|&&l| a < l).count() - 1;
    println!("{}", height[i]);
  }
}
