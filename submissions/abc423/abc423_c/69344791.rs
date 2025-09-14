use proconio::input;

fn main() {
  input! {
    n: usize,
    r: usize,
    l: [usize; n],
  }

  if let Some(left_option) = l.iter().position(|&li| li == 0) {
    let left = left_option.min(r);
    let mut right = l.iter().rposition(|&li| li == 0).unwrap();
    if right >= r {
      right = r - 1;
    }
    let range = right - left + 1;
    let count = l[left + 1..right].iter().filter(|&&li| li == 1).count();
    println!("{}", count + range);
  } else {
    println!("0");
  }
}
