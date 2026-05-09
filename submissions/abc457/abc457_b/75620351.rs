use text_io::read;

fn main() {
  let n: usize = read!();

  let mut matrix = Vec::new();
  for _ in 0..n {
    let l: usize = read!();
    let mut a = Vec::new();
    for _ in 0..l {
      let i: usize = read!();
      a.push(i);
    }
    matrix.push(a);
  }

  let x: usize = read!();
  let y: usize = read!();

  println!("{}", matrix[x - 1][y - 1]);
}
