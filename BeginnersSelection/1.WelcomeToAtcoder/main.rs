use std::io;

fn main() {
    // 入力から各値を取得
    let mut input = String::new();
    
    // a の入力
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let a: i32 = input.trim().parse().expect("Please enter a valid integer");
    
    input.clear();
    
    // b の入力
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let b: i32 = input.trim().parse().expect("Please enter a valid integer");
    
    input.clear();
    
    // c の入力
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let c: i32 = input.trim().parse().expect("Please enter a valid integer");
    
    input.clear();
    
    // s の入力
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let s = input.trim();

    // a + b + c の計算
    let sum = a + b + c;
    
    // 出力
    println!("{} {}", sum, s);
}