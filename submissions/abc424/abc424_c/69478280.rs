use proconio::input;

fn main() {
  input! {
    n: usize,
    ab: [(usize, usize); n]
  }

  let mut skills = vec![false; n];
  for (i, &(a, b)) in ab.iter().enumerate() {
    if (a == 0 && b == 0) || skills[a - 1] || skills[b - 1] {
      skills[i] = true;
    }
  }

  println!("{}", skills.iter().filter(|&&i| i).count());
}
