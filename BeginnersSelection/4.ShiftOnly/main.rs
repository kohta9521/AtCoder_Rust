use proconio::input;

// ABC081B - Shift only
// https://atcoder.jp/contests/abs/tasks/abc081_b

fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
    }

    let mut count = 0;

    loop {
        if a.iter().any(|&x| x % 2 != 0) {
            break;
        }

        for i in 0..n {
            a[i] /= 2;
        }

        count += 1;
    }

    println!("{}", count);
}