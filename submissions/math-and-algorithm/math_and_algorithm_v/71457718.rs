use proconio::input;

fn main() {
  input! {
      n: usize,
      a: [usize; n],
  }

  let mut counter = vec![0usize; 100000];

  for x in a {
    counter[x] += 1;
  }

  let mut ans = 0usize;

  for i in 1..50000 {
    ans += counter[i] * counter[100000 - i];
  }

  ans += counter[50000] * (counter[50000] - 1) / 2;

  println!("{}", ans);
}
