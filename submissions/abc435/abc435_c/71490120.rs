use bitvec::order::Lsb0;
use bitvec::vec::BitVec;
use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [usize; n],
  }

  let mut taoreru = BitVec::<usize, Lsb0>::repeat(false, n);
  taoreru.set(0, true);

  for (&h, cnt) in a.iter().zip(0..n) {
    if taoreru[cnt] {
      for i in 0..h {
        if cnt + i < n {
          taoreru.set(cnt + i, true);
        }
      }
    }
  }

  println!(
    "{}",
    taoreru.into_iter().position(|x| x == false).unwrap_or(n)
  );
}
