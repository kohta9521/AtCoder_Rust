fn digit_sum(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    let last_digit = n % 10;
    let higher_digits_sum = degit_sum(n / 10);
    let result = higher_digits_sum + last_digit;
    println!(
        "{} の各行の和は {} + {} = {}",
        n, higher_digits_sum, last_digit, result
    );
    result
}


fn main() {
    assert_eq!(digit_sum(6318), 18)
}