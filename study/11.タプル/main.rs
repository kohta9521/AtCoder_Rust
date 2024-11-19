fn main() {
    let tuple: (i32, f64, i32) = (10, 2.5, 20);
    println!("1番目の要素:{}", tuple.0);
    println!("2番目の要素:{}", tuple.1);
    println!("3番目の要素:{}", tuple.2);

    // パターンマッチ
    let tuple = (10, 2.5);
    let (x, y) = tuple;
    assert_qu!(x, 10);
    assert_qu!(y, 2.5);

    // ユニット
    let unit = ();
    unit = {
        println!("() を返すブロック");
    };
    assert_eq!(unit, ());
}