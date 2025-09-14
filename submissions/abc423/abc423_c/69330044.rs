use proconio::input;

fn main() {
  input! {
    n: usize,
    r: usize,
    l: [usize; n],
  }

  let left = l.iter().position(|&li| li == 0).unwrap_or(0).min(r);
  let right = l.iter().rposition(|&li| li == 0).unwrap_or(0).max(r);
  let range = right - left + 1;
  let count = l[left..right].iter().filter(|&&li| li == 1).count();

  println!("{}", count + range);
}
