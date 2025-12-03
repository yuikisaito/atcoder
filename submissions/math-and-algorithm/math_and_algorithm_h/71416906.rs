use proconio::input;

fn main() {
  input! {
    n: usize,
    s: usize,
  }

  if s <= n + 1 {
    let k = s - 1;
    println!("{}", k * (k + 1) / 2);
  } else if s < 2 * n {
    let rect = n * (s - n - 1);
    let tri = n * (n + 1) / 2;
    let over_tri = (s - n - 1) * ((s - n - 1) + 1) / 2;
    println!("{}", rect + tri - over_tri);
  } else {
    println!("{}", n.pow(2))
  }
}
