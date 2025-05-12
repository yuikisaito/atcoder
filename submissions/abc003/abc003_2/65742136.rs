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
            if sis == "@"
                && (tis == "a"
                    || tis == "t"
                    || tis == "c"
                    || tis == "o"
                    || tis == "d"
                    || tis == "e"
                    || tis == "r"
                    || tis == "@")
            {
                continue;
            }
            if tis == "@"
                && (sis == "a"
                    || sis == "t"
                    || sis == "c"
                    || sis == "o"
                    || sis == "d"
                    || sis == "e"
                    || sis == "r"
                    || sis == "@")
            {
                continue;
            }
            println!("You will lose");
            return;
        }
    }
    println!("You can win")
}
