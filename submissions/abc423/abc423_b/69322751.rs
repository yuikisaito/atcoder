use proconio::input;

fn main() {
  input! {
    n: usize,
    l: [usize; n],
  }

  let left = l.iter().position(|&li| li == 1).unwrap_or(n - 1) + 1;
  let right = ((n - 1) - l.iter().rposition(|&li| li == 1).unwrap_or(n - 1)) + 1;

  println!("{}", (left + right).min(n + 1));
}
