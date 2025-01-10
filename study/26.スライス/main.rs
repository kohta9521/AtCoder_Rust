fn main() {
    let mut ref_slice: &[i32];

    let array = [1, 2, 3];
    ref_slice = &array;
    println!("{:?}", ref_slice);

    let vector = vec![4, 5, 6];
    ref_slice = &vector;
    println!("{:?}", ref_slice);
}