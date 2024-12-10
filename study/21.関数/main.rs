fn main() {
    let value = {
        let mut n = 1;
        for i in 1..=6 {
            print!("{} ", n);
            n *= i;
        }
        println!("{}", n);
        n
    }
    assert_eq!(value, 120);
}