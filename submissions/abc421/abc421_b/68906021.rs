use proconio::input;

fn main() {
  input! {
    x: usize,
    y: usize,
  }

  let mut n = x;
  let mut m = y;

  for _ in 3..=10 {
    let sum = n + m;
    let rev = sum
      .to_string()
      .chars()
      .rev()
      .collect::<String>()
      .parse::<usize>()
      .unwrap();
    n = m;
    m = rev;
  }

  println!("{}", m);
}
