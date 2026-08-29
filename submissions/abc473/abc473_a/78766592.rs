use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [usize; n],
  }

  println!("{}", a.into_iter().skip(n / 2).sum::<usize>());
}
