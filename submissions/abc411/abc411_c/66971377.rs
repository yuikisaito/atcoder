use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; q],
    }

    let mut change = Vec::<usize>::new();

    let mut prev_cnt = 0;
    for x in a {
        let mut cnt = 0;
        if let Some(ri) = change.iter().position(|&s| s == x) {
            change.remove(ri);
            println!("{}", prev_cnt);
            continue;
        }
        change.push(x);
        change.sort();
        change.iter().fold(n + 1, |acc, &y| {
            if acc + 1 != y {
                cnt += 1;
            }
            y
        });
        println!("{}", cnt);
        prev_cnt = cnt;
    }
}
