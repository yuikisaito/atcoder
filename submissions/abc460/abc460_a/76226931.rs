use proconio::input;

fn main() {
  input! {
    n: usize,
    mut m: usize,
  }

  let mut cnt =0;
  while m != 0 {
    m = n % m;
    cnt += 1;
  }

  println!("{}", cnt);
}
