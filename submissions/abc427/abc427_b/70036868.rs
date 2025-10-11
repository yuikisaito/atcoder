use proconio::input;

fn main() {
  input! {
    n: usize,
  }

  let mut sum = 0;
  for i in 0..n {
    sum += if i == 0 {
      1
    } else {
      let ai = sum
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum::<u32>();
      ai
    }
  }

  println!("{}", sum);
}
