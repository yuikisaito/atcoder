use proconio::input;

fn main() {
    input! {
        t: isize,
        n: usize,
        a: [isize; n],
        m: usize,
        b: [isize; m]
    }

    let mut a_iter = a.iter();

    let msg = if b
        .iter()
        .all(|bi| a_iter.find(|&ai| ai <= bi && bi - ai <= t).is_some())
    {
        "yes"
    } else {
        "no"
    };

    println!("{}", msg);
}
