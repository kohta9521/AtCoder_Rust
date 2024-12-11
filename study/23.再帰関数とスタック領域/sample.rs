fn gcd(m: i32, n: i32) -> i32 {
    if n == 0 {
        m
    } else {
        gcd(n, m % n)
    }
}

fn main() {
    assert_eq!(gcd(18, 30), 6);
    assert_eq!(gcd(30, 18), 6);
}