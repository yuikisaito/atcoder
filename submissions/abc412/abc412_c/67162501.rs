use proconio::input;

fn main() {
    input! {
        t: usize,
        case: [[usize]; t]
    }

    for s in case {
        let mut new_s = Vec::new();
        new_s.push(*s.first().unwrap());
        let mut sorted_s = s[1..s.len() - 1].to_vec();
        sorted_s.sort();
        new_s.append(&mut sorted_s);
        new_s.push(*s.last().unwrap());

        let mut acc = new_s[0].clone();
        let mut end = false;
        for &si in &new_s {
            if !(acc * 2 >= si) {
                end = true;
                break;
            }
            acc = si;
        }
        if end {
            println!("-1");
            continue;
        }

        let mut prev_cnt = 0;
        let mut prev_n = new_s[0].clone();
        let mut cnt = 1;
        for si in 0..new_s.len() {
            if prev_n * 2 >= new_s[si] {
                prev_cnt += 1;
            } else {
                cnt += prev_cnt;
                prev_cnt = 1;
                prev_n = new_s[si - 1];
            }
        }
        println!("{}", cnt);
    }
}
