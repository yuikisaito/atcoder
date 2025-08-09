use proconio::input;

fn main() {
  input! {
    n: usize,
    q: usize,
    a: [usize; n],
    b: [usize; q],
  }

  let total_a = a.iter().sum();
  let max_a = *a.iter().max().unwrap();
  for bi in b {
    if bi > max_a {
      println!("-1");
      continue;
    }
    let dealer_win = a.iter().map(|&ai| usize::min(bi - 1, ai)).sum::<usize>();
    if dealer_win >= total_a {
      println!("-1")
    } else {
      println!("{}", dealer_win + 1);
    }
  }
}
