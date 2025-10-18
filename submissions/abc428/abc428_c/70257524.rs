use text_io::read;
fn main() {
  let q: usize = read!();
  let mut s = String::new();
  let judge = |str: &String| -> bool {
    let len = str.len();
    let divide = len / 2;
    if len % 2 != 0 {
      return false
    }
    if str.chars().take(divide).all(|c| c == '(') && str.chars().skip(divide).all(|c| c == ')') {
      return true;
    }
    return false
  };
  for _ in 0..q {
    let query: usize = read!();
    match query {
      1 => {
        s.push(read!());
      },
      2 => {
        s.remove(s.len() - 1);
      },
      _ => unreachable!(),
    }
    if judge(&s) {
      println!("Yes");
    } else {
      println!("No");
    }
  }
}
