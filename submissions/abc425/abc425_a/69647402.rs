use proconio::input;

fn main() {
  input! {
    n: usize,
  }

  let sum = (1..=n)
    .map(|i| (-1isize).pow(i as u32) * i.pow(3) as isize)
    .sum::<isize>();

  println!("{}", sum);
}
