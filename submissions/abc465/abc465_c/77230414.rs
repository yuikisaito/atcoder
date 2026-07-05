use itertools::izip;
use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    _n: usize,
    s: Chars,
  }

  let mut front = Vec::new();
  let mut back = Vec::new();
  let mut rev = false;

  for (c, n) in izip!(s, 1..) {
    if rev {
      front.push(n.to_string());
    } else {
      back.push(n.to_string());
    }
    rev ^= c == 'o';
  }

  front.reverse();
  let mut ans = front;
  ans.extend(back);
  if rev {
    ans.reverse();
  }

  println!("{}", ans.join(" "));
}
