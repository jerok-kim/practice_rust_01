fn main() {
    // let x: i8 = 10;
    // println!("{}", x);
    // 
    // let y: u8 = 10;
    // 
    // let decimal = 02_55;
    // let hex = 0xff;
    // let octal = 0o377;
    // let binary = 0b1111_1111;
    // 
    // println!("{}", decimal);
    // println!("{}", hex);
    // println!("{}", octal);
    // println!("{}", binary);
    // 
    // let byte = b'A';
    // println!("{}", byte);

    // let x = 2.0;  // f64 default because on modern CPUs
    // let y: f32 = 1.0;
    // 
    // let t = true;
    // let f: bool = false;
    // 
    // let c = 'c';
    // println!("{}", c);
    // 
    // // + - * / %
    // let a = 10;
    // let b = 4;
    // 
    // let remainder = a % b;
    // println!("{}", remainder);

    // let tup = (500, "hi", true);
    // println!("{} {} {}", tup.0, tup.1, tup.2);
    // 
    // let (x, y, z) = tup;
    // println!("{} {} {}", x, y, z);

    // let array = [1, 2, 3];
    // println!("{} {} {}", array[0], array[1], array[2]);
    // 
    // let mut array2: [i32; 3] = [4, 5, 6];
    // println!("{} {} {}", array2[0], array2[1], array2[2]);
    // 
    // array2[0] = 10;
    // println!("{}", array2[0]);
    // println!("{}", array2[3]);  // panic

    let mut nums = vec![1, 2, 3];

    nums.push(4);
    println!("{:?}", nums);

    nums.pop();
    println!("{:?}", nums);

    let mut vec = Vec::new();
    vec.push("Test");
    vec.push("String");
    println!("{:?}", vec);

    vec.reverse();
    println!("{:?}", vec);

    let mut vect = Vec::<i32>::with_capacity(2);
    println!("{:?}", vect);
    println!("{}", vect.capacity());

    let v: Vec<i32> = (0..5).collect();
    println!("{:?}", v);
}
