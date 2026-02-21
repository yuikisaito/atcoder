use text_io::read;
fn main() {
  let n: usize = read!();
  let m: usize = read!();
  let mut used = vec![false; m];

  for _ in 0..n {
    let l: usize = read!();
    let mut returned = false;
    for _ in 0..l {
      let x: usize = read!();

      if !returned && !used[x - 1] {
        println!("{}", x);
        used[x - 1] = true;
        returned = true;
      }
    }

    if !returned {
      println!("0");
    }
  }
}
