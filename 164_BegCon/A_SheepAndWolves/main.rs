fn main() {
    proconio::input! {
        s: i32,
        w: i32,
    }
    if s <= w {
        println!("unsafe");
    } else {
        println!("safe");
    }
}