use proconio::input;

fn main() {
  input! {
    n: usize,
    q: usize,
    queries: [(usize, usize); q],
  }

  let mut cells = vec![0; n];
  let mut remove = 0;
  for (t, x) in queries {
    match t {
      1 => {
        cells[x - 1] += 1;
        if cells.iter().all(|&c| c - remove >= 1) {
          remove += 1;
        }
      },
      2 => {
        println!("{}", cells.iter().filter(|&&c| c >= x + remove).count());
      },
      _ => unreachable!(),
    }
  }
}
