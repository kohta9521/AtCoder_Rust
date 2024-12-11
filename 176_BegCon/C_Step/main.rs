use proconio::input;

fn main() {
    input! {
        a: [i64],
    }
    // 前にいる人の身長の最大値
    let mut max = 0;
    let mut ans = 0;
    for i in &a {
        max = max.max(*i);
        ans += max - i;
    }
    println("{}", ans);
}