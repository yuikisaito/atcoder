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
        let dist = (x as i32 - prev_x as i32).abs() + (y as i32 - prev_y as i32).abs();

        if dist > time_diff as i32 || dist % 2 != time_diff as i32 % 2 {
            println!("No");
            return;
        }

        prev_time = time;
        prev_x = x;
        prev_y = y;
    }

    println!("Yes");
}
