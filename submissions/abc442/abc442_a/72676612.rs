use proconio::{input, marker::Chars};

fn main() {
  input! {
    s: Chars,
  }

  println!(
    "{}",
    s.into_iter().filter(|c| ['i', 'j'].contains(c)).count()
  );
}
