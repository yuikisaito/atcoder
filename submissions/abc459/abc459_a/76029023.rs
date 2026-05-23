use proconio::input;
use proconio::marker::Usize1;

fn main() {
  input! {
    x: Usize1,
  }

  println!("{}", "HelloWorld".chars().into_iter().enumerate().filter_map(|(i, c)| if i != x { Some(c) } else { None }).collect::<String>());
}
