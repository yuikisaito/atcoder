use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [usize; n],
  }

  println!("{}", a.into_iter().enumerate().filter_map(|(i, x)| if i % 2 == 0 { Some(x) } else { None }).sum::<usize>());
}
