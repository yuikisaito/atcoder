use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        d: [usize; n - 1]
    }

    if l % 3 != 0 {
        println!("0");
        return;
    }

    let mut points = vec![Vec::<usize>::new(); l];
    points[0].push(1);
    let mut prev = 0;
    for (index, di) in d.iter().enumerate() {
        let pi = (prev + *di) % l;
        points[pi].push(index + 2);
        prev = pi;
    }

    let ippen = l / 3;
    let sankakukei_points: usize = (0..ippen)
        .map(|p| vec![p, p + ippen, p + ippen * 2])
        .map(|p| p.iter().fold(1, |prev, i| prev as usize * points[*i].len()))
        .sum();

    println!("{}", sankakukei_points);
}
