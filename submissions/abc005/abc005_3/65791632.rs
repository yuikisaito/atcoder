use proconio::input;

fn main() {
    input! {
        t: isize,
        n: isize,
        a: [isize; n],
        m: isize,
        b: [isize; m]
    }

    let msg = if a
        .iter()
        .zip(b.iter())
        .all(|(a, b)| 0 <= b - a && b - a <= t)
        && a.len() >= b.len()
    {
        "yes"
    } else {
        "no"
    };
    println!("{}", msg)
}
