use proconio::input;

fn main() {
  input! {
    n: usize,
    l: [isize; n],
  }

  let mut pos = 0;
  let mut nega = false;
  let mut cnt = 0;
  for i in l {
    if pos >= 0 {
      pos -= i
    } else {
      pos += i;
    }
    if nega != pos.is_negative() {
      cnt += 1;
      nega = !nega;
    }
  }

  println!("{}", cnt);
}
