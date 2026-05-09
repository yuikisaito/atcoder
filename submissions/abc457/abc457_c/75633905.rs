use text_io::read;

fn main() {
  let n: usize = read!();
  let mut k: usize = read!();
  k -= 1;

  let mut matrix = Vec::new();
  let mut columns = Vec::new();
  for _ in 0..n {
    let l: usize = read!();
    columns.push(l);
    let mut a = Vec::new();
    for _ in 0..l {
      let i: usize = read!();
      a.push(i);
    }
    matrix.push(a);
  }

  for i in 0..n {
    let c: usize = read!();
    if let Some(result) = k.checked_sub(columns[i] * c) {
      k = result
    } else {
      println!("{}", matrix[i][k % columns[i]]);
      return;
    }
  }
}
