use proconio::input;

fn main() {
  input! {
    n: usize,
    k: usize,
  }

  let mut sum = 0;
  for i in 0.. {
    sum += n + i;
    if sum >= k {
      println!("{}", i);
      return
    }
  }
}
