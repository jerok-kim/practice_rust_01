/// section 3 - assignment

fn main() {
    let mut vector = vec![1, 3, 5, 7];
    is_one(&vector);
    vector.push(15);
    println!("{:?}", vector);

    println!("{}", add_two(3));
}

fn is_one(val: &Vec<i32>) -> bool {
    if val[0] == 1 {
        true
    } else {
        false
    }
}

fn add_two(num: i8) -> i8 {
    num + 2
}
