use text_io::read;
fn main() {
  let q: usize = read!();
  let mut s = String::new();
  let mut open = 0;
  let mut close = 0;
  let mut first_close = isize::MAX;
  let mut idx: isize = -1;
  for _ in 0..q {
    let query: usize = read!();
    match query {
      1 => {
        let c: char = read!();
        match c {
          '(' => open += 1,
          ')' => close += 1,
          _ => unreachable!(),
        }
        s.push(c);
        idx += 1;
      },
      2 => {
        let last_idx = s.len() - 1;
        let c = s.chars().nth(last_idx).unwrap();
        match c {
          '(' => open -= 1,
          ')' => close -= 1,
          _ => unreachable!(),
        }
        s.remove(last_idx);
        idx -= 0;
      },
      _ => unreachable!(),
    }
    println!("{}, {}", idx, first_close);
    if idx <= first_close {
      first_close = if close > open { idx } else { isize::MAX }
    };
    println!("{}, {}", idx, first_close);
    if open != close || idx >= first_close {
      println!("No");
      continue;
    }
    println!("Yes");
  }
}
