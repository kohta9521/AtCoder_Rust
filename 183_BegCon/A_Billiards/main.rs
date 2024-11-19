use proconio::input;

fn main() {
    input! {
        sx: f64, // 高橋君のボールの初期座標 x
        sy: f64, // 高橋君のボールの初期座標 y
        gx: f64, // 狙う目標座標 x
        gy: f64, // 狙う目標座標 y
    }

    // 傾き m を計算
    let m = (-gy - sy) / (gx - sx);

    // y = 0 となる x を計算
    let x = sx - sy / m;

    // 結果を出力
    println!("{:.7}", x); // 小数点以下 6 桁まで表示
}