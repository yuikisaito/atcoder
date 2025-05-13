use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let can_win = s.iter().zip(t.iter()).all(|(&s, &t)| {
        s == t || (s == '@' && "atcoder@".contains(t)) || (t == '@' && "atcoder@".contains(s))
    });

    if can_win {
        println!("You can win");
    } else {
        println!("You will lose");
    }
}
