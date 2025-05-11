use proconio::input;

fn main() {
    input! {
        n: usize,
        points: [(u32, u32, u32); n],
    }

    let mut prev_time = 0;
    let mut prev_x = 0;
    let mut prev_y = 0;

    for (time, x, y) in points {
        let time_diff = time - prev_time;
        let dist = (x - prev_x) + (y - prev_y);

        if dist > time_diff || dist % 2 != time_diff % 2 {
            println!("No");
            return;
        }

        prev_time = time;
        prev_x = x;
        prev_y = y;
    }

    println!("Yes");
}
