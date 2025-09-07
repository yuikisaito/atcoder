use proconio::input;

fn main() {
  input! {
    t: usize,
    cases: [[usize; 3]; t]
  }

  for case in cases {
    let (a, b, c) = (case[0], case[1], case[2]);
    if b < a && b < c {
      let ac_ave = ((a - b) + (c - b)) / 3;
      let min = ac_ave.min(a - b).min(c - b);
      println!("{}", b + min);
    } else {
      let ac_min = c.min(a);
      let ac_max = a.min(c);
      let other = ac_max + b - ac_min;
      println!("{}", other.min(ac_min));
    }
  }
}
