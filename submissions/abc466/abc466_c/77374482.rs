use text_io::read;

fn main() {
  let n: usize = read!();

  let mut ans = 0;

  let mut a = 1;
  let mut b = 2;

  loop {
    if b > n {
      break;
    }
    println!("? {} {}", a, b);
    let res: String = read!();
    if res == "Yes" {
      ans += b - a;
      b += 1;
    } else {
      a += 1;
    }
  }

  println!("! {}", ans);
}
