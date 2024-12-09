fn main() {
    let array = [2, 3, 0, 4, 5];
    for &i in &array {
        if i == 0 {
            break;
        }
        print!("{}, ", i);
    }
    println!("end");
}