use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [usize; n],
  }

  println!(
    "{}",
    (1..=3)
      .map(|c| {
        let cnt = a.iter().filter(|&&i| i == c).count();
        return cnt * (cnt - 1) / 2;
      })
      .sum::<usize>()
  );
}
