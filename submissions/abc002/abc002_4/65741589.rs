use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        cons: [(usize, usize); m]
    }

    let mut members = vec![vec![false; n]; n];
    for con in cons {
        members[con.0 - 1][con.1 - 1] = true;
    }

    let mut max_group_members = 0;

    for k in 1..=n {
        for group in (0..n).combinations(k) {
            let mut valid_group = true;
            // iとjの二人を順番に処理
            'group: for i in 0..group.len() {
                for j in (i + 1)..group.len() {
                    let u = group[i];
                    let v = group[j];
                    if !members[u][v] {
                        valid_group = false;
                        break 'group;
                    }
                }
            }
            if valid_group {
                max_group_members = std::cmp::max(max_group_members, group.len());
            }
        }
    }
    println!("{}", max_group_members)
}
