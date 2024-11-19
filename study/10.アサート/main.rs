use proconio::input;

fn main() {
    input! {
        x: i32,
    }
    let r = x % 10;
    assert!(0 <= r && r < 10, "割った余りが {} で、想定の範囲を超えています", r);
    println!("あまりは{}", r);
}