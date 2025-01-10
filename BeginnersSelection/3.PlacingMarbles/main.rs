use proconio::input;

// ABC086 - traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        s: String,
    }

    println!("{}", s.matches("1").count());
}