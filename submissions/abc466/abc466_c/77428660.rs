use proconio::input_interactive;

fn main() {
  input_interactive! {
    n: usize
  }

  let mut ans = 0;

  let mut a = 1;
  let mut b = 2;

  while b <= n {
    println!("? {} {}", a, b);

    input_interactive! {
      res: String
    }

    if res == "Yes" {
      ans += b - a;
      b += 1;
    } else {
      a += 1;
    }

    if a == b {
      b += 1;
    }
  }

  println!("! {}", ans);
}
