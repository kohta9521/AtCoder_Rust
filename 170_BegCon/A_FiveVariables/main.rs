fn main() {
    proconio::input! {
        a1: i32,
        a2: i32,
        a3: i32,
        a4: i32,
        a5: i32,
    }
    println!(
        "{}",
        if a1 == 0 {
            1
        } else if a2 == 0 {
            2
        } else if a3 == 0 {
            3
        } else if a4 == 0 {
            4
        } else {
            5
        }
    );
}