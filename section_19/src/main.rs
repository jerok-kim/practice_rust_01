fn main() {
    let vec = vec![1, 2, 3, 4, 5];

    let mut max = 0;

    for e in vec.clone() {
        if e > max { max = e; }
    }

    println!("{}", max);
    println!("{:?}", vec);
}
