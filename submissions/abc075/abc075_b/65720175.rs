use proconio::{input, marker::Chars};

fn main() {
    // 入力読み込み: H 行, W 列のグリッド
    input! {
        h: usize,
        w: usize,
        grid: [Chars; h],
    }

    // 隣接 8 マスの変位ベクトル
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    // 各セルを走査
    for row in 0..h {
        // 1 行分の結果を構築
        let mut output_line = String::with_capacity(w);

        for col in 0..w {
            match grid[row][col] {
                '#' => {
                    // 爆弾セルはそのまま '#'
                    output_line.push('#');
                }
                '.' => {
                    // 空きセル: 8 方向の爆弾数をカウント
                    let mut count = 0;
                    for &(dr, dc) in &directions {
                        let nr = row as isize + dr;
                        let nc = col as isize + dc;
                        if nr >= 0 && nr < h as isize && nc >= 0 && nc < w as isize {
                            if grid[nr as usize][nc as usize] == '#' {
                                count += 1;
                            }
                        }
                    }
                    // 数字 ('0' + count) を文字として追加
                    output_line.push(char::from(b'0' + count as u8));
                }
                _ => unreachable!(),
            }
        }

        // 1 行を出力
        println!("{}", output_line);
    }
}
