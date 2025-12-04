use proconio::input;

fn main() {
  input! {
    n: usize,
    s: usize,
    a: [usize; n],
  }

  let mut table = vec![false; s + 1];
  table[0] = true;

  for x in a {
    for j in (x..s + 1).rev() {
      if table[j - x] {
        table[j] = true;
      }
    }
  }

  println!("{}", if table[s] { "Yes" } else { "No" })
}
