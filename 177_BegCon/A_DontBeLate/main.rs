fn main() {
    proconio::input! {
        d: i32,
        t: i32,
        s: i32,
    }
    if t * s >= d {
        println!("Yes");
    } else {
        println!("No");
    }
}