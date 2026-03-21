use text_io::read;

fn main() {
  let n: usize = read!();
  let mut c = Vec::new();
  for i in 0..n - 1 {
    let mut row = Vec::new();
    for _ in i..n - 1 {
      let ci: usize = read!();
      row.push(ci);
    }
    c.push(row);
  }

  println!(
    "{}",
    if (0..n).any(|from| (from + 2..n).any(|to| (from + 1..to).any(|via| {
      eprintln!("{} {} {}", from, to, via);
      c[from][to - (from + 1)] > c[from][via - (from + 1)] + c[via][to - (via + 1)]
    }))) {
      "Yes"
    } else {
      "No"
    }
  );
}
