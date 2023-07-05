fn main() {
    let vec = vec![1, 3, 5, 7, 9];
    let result: Vec<i32> = vec.iter().map(|x| x * 10).collect();
    println!("{:?}", result);
}
