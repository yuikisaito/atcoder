use proconio::input;

fn main() {
  input! {
    n: usize,
    q: usize,
    queries: [(usize, usize); q],
  }

  let mut cells = vec![0usize; n];
  for (t, x) in queries {
    match t {
      1 => {
        cells[x - 1] += 1;
        if cells.iter().all(|&c| c >= 1) {
          cells.iter_mut().for_each(|c| *c -= 1);
        }
      },
      2 => {
        println!("{}", cells.iter().filter(|&&c| c >= x).count());
      },
      _ => unreachable!(),
    }
  }
}
