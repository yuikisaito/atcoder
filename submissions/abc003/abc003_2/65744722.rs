use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    for i in 0..s.len() {
        let si = s[i];
        let ti = t[i];
        if si != ti {
            let sis = si.to_string();
            let tis = ti.to_string();
            if sis == "@" && "atcoder@".contains(&tis) {
                continue;
            }
            if tis == "@" && "atcoder@".contains(&sis) {
                continue;
            }
            println!("You will lose");
            return;
        }
    }
    println!("You can win")
}
