use bitvec::order::Lsb0;
use bitvec::vec::BitVec;
use itertools::Itertools;
use num_integer::sqrt;
use proconio::input;

fn main() {
  input! {
    n: usize,
  }

  let mut bits = BitVec::<usize, Lsb0>::repeat(true, n + 1);
  for i in 2..=sqrt(n) {
    if bits[i] {
      for j in (i + 1..n + 1).filter(|x| x % i == 0) {
        bits.set(j, false);
      }
    }
  }

  println!(
    "{}",
    bits
      .into_iter()
      .enumerate()
      .filter(|&(i, x)| x && i >= 2)
      .map(|(i, _)| i)
      .join(" ")
  );
}
