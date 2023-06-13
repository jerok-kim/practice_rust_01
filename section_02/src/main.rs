use std::ops::Add;

/// Section 2 - assignment

fn main() {
    // 1
    let val1 = 5;
    let val2 = 2;
    let ans = val1 % val2;

    // 2
    let mut vec = vec![2, 4, 6, 8, 10];
    println!("Current vector: {:?}", vec);
    vec.retain(|&x| x != 10);
    vec.push(12);
    println!("Modified vector: {:?}", vec);

    // 3
    let result = concat_string(String::from("World"));
    println!("{}", result);

    control_flow(1);
    control_flow(25);
    control_flow(50);

}

fn concat_string(arg: String) -> String {
    let hello = String::from("Hello");
    hello.add(arg.as_str())
}

// 4
fn control_flow(num: i32) {
    if num == 1 {
        println!("The value is one");
    } else if num > 50 {
        println!("The value is greater than 50");
    } else if num < 25 {
        println!("The value is less than 25");
    } else if num > 25 && num < 50 {
        println!("The value is greater than 25 but less than 50");
    } else {
        println!("The value is 50 or 25");
    }
}
