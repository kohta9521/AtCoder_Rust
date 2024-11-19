// use proconio::input;

fn main() {
    // input! {
    //     x: i32,
    // }
    // if x > 0 {
    //     println!("正");
    // }

    let hoge = 10;
    {
        println!("{}", hoge);
        let hoge = 20;
        println!("{}", hoge);
    }
    println!("{}", hoge);

    let hoge: i32 = 10;
    println!("{}", hoge);
    let hoge: f64 = 20.;
    println!("{}", hoge);
}