use proconio::input;

fn main() {
    input! {
        m: usize,
    }

    let vv;
    if m < 100 {
        vv = 0;
    } else if m <= 5000 {
        vv = m * 10 / 1000;
    } else if m <= 30000 {
        vv = m / 1000 + 50;
    } else if m <= 70000 {
        vv = (m / 1000 - 30) / 5 + 80;
    } else {
        vv = 89;
    }

    println!("{:02}", vv);
}
