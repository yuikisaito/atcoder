use proconio::input;

fn main() {
  input! {
    q: usize,
    a: [usize; q],
  }

  let mut vol = 0usize;
  let mut playing = false;
  for i in a {
    match i {
      1 => vol += 1,
      2 => vol = vol.saturating_sub(1),
      3 => playing = !playing,
      _ => unreachable!(),
    }
    println!("{}", if playing && vol >= 3 { "Yes" } else { "No" });
  }
}
