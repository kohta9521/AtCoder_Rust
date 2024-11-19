fn main() {
    let array: [i64; 5];
    array = [3, 7, 31, 127, 8191];
    assert_eq!(array[0], 3_i64);
    assert_eq!(array[1], 7_i64);
    assert_eq!(array[2], 31_i64);
    assert_eq!(array[3], 127_i64);
    assert_eq!(array[4], 8191_i64);
}