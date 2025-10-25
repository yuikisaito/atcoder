use proconio::input;

fn main() {
  input! {
    n: usize,
    m: usize,
    a: [usize; n],
  }

  let sum: usize = a.iter().sum();

  for i in 0..n {
    if sum - a[i] == m {
      println!("Yes");
      return
    }
  }

  println!("No");
}
