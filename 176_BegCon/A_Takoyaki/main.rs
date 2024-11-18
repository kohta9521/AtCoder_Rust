use proconio::input;

fn main() {
    input! {
        n: i32,
        x: i32,
        t: i32,
    }
    // 仮で設定：x = 12 n = 20 t = 6
    // 12 / 20 = 0.6
    println!("{}", ((n - 1) / x + 1));
}