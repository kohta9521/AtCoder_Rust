use proconio::input;

// ABC087B - Coins
// https://atcoder.jp/contests/abs/tasks/abc087_b

fn main() {
    // 標準入力の受け取り
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    }

    // 最終出力の回数をカウントする変数
    let mut count = 0;

    // 500円だまのループ
    for i in 0..=a {
        // 100円だまのループ
        for j in 0..=b {
            // 50円玉のループ
            for k in 0..=c {
                // 合計金額の計算
                let total = 500 * i + 100 * j + 50 * k;

                // 合計金額が入力値と一致した場合にカウント変数へ加算
                if total == x {
                    count += 1;
                }
            }
        }
    }

    // 最終出力
    println!("{}", count);
}