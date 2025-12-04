use num_integer::sqrt;
use proconio::input;

fn main() {
  input! {
    n: usize,
  }

  let mut lower = Vec::new();
  let mut upper = Vec::new();
  let mut i = 1;
  while i <= sqrt(n) {
    if n % i == 0 {
      lower.push(i);
      if i != n / i {
        upper.push(n / i);
      }
    }
    i += 1;
  }

  for l in lower.into_iter().chain(upper.into_iter().rev()) {
    println!("{}", l);
  }
}
