use std::f32::consts::PI;

use proconio::input;

fn main() {
  input! {
    d: f32,
  }

  println!("{}", (d / 2.).powf(2.) * PI);
}
