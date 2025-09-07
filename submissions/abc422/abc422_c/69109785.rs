use proconio::input;

fn main() {
  input! {
    t: usize,
    cases: [[usize; 3]; t]
  }

  for case in cases {
    let (a, b, c) = (case[0], case[1], case[2]);
    let ac_min = a.min(c);
    let ac_max = a.max(c);
    let other = ac_max + b - ac_min;
    println!("{}", other.min(ac_min));
  }
}
