use proconio::input;

fn main() {
    input! {
        deg: usize,
        dis: usize,
    }

    let speed = (dis as f64 / 60.0 * 10.0).round() / 10.0;

    let w = match speed {
        x if x <= 0.2 => 0,
        x if x <= 1.5 => 1,
        x if x <= 3.3 => 2,
        x if x <= 5.4 => 3,
        x if x <= 7.9 => 4,
        x if x <= 10.7 => 5,
        x if x <= 13.8 => 6,
        x if x <= 17.1 => 7,
        x if x <= 20.7 => 8,
        x if x <= 24.4 => 9,
        x if x <= 28.4 => 10,
        x if x <= 32.6 => 11,
        _ => 12,
    };

    let dir = if w == 0 {
        "C"
    } else {
        const DIRS: [&str; 32] = [
            "N", "NNE", "NNE", "NE", "NE", "ENE", "ENE", "E", "E", "ESE", "ESE", "SE", "SE", "SSE",
            "SSE", "S", "S", "SSW", "SSW", "SW", "SW", "WSW", "WSW", "W", "W", "WNW", "WNW", "NW",
            "NW", "NNW", "NNW", "N",
        ];
        let sector = 3600.0 / 32.0;
        let idx = (deg as f64 / sector).round() as usize;
        DIRS[idx]
    };

    println!("{} {}", dir, w);
}
