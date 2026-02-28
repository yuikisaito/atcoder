use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    s: Chars,
    t: Chars,
  }

  let partition = |chars: Vec<_>| {
    let mut a_pos = Vec::new();
    let mut not_a = Vec::new();
    let mut crnt = 0;
    for c in chars.into_iter() {
      if c == 'A' {
        crnt += 1;
      } else {
        not_a.push(c);
        a_pos.push(crnt);
        crnt = 0;
      }
    }
    a_pos.push(crnt);
    (a_pos, not_a)
  };

  let (s_a_pos, s_not_a) = partition(s);
  let (t_a_pos, t_not_a) = partition(t);

  if s_not_a != t_not_a {
    println!("-1");
    return
  }

  let mut count = 0;
  for i in 0..=s_not_a.len() {
    let d: i32 = s_a_pos[i] - t_a_pos[i];
    count += d.abs();
  }

  println!("{}", count);
}
