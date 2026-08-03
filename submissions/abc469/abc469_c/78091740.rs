use std::io::BufWriter;
use std::io::Write;
use std::io::{
  self,
};

use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    n: usize,
    s: Chars,
  }

  let mut s_vec = Vec::<isize>::with_capacity(n);
  let mut change = Vec::<isize>::with_capacity(n);

  unsafe {
    s_vec.set_len(n);
    change.set_len(n);

    let mut current_state: isize = 0;
    let s_ptr = s_vec.as_mut_ptr();
    let change_ptr = change.as_mut_ptr();

    for i in 0..n {
      let val = (*s.get_unchecked(i) == 'o') as isize;
      *s_ptr.add(i) = val;

      current_state += val - 1;
      *change_ptr.add(i) = current_state;
    }

    let stdout = io::stdout();
    let mut handle = BufWriter::new(stdout.lock());

    let mut sum: isize = 0;
    let mut curr_search_idx = 0;

    for k in 0..n {
      sum += *s_ptr.add(k);
      let goal = *change_ptr.add(k) - sum;

      while curr_search_idx < n {
        if *change_ptr.add(curr_search_idx) <= goal {
          curr_search_idx += 1;
          break;
        }
        curr_search_idx += 1;
      }

      let ans = n.min(curr_search_idx);
      let _ = writeln!(handle, "{}", ans);
      println!()
    }
  }
}
