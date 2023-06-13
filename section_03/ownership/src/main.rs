fn main() {

    // let var = 1;  // on the stack
    // let mut s = "Hello".to_string();  // created on the heap
    // s.push_str(", world");

    // let x = vec!["jerok".to_string()];
    // let y = x;
    // println!("{:?}", y);

    // let x = vec!["jerok".to_string()];
    // let y = x.clone();
    // let z = y.clone();
    // println!("{:?}", x);
    // println!("{:?}", y);
    // println!("{:?}", z);

    // let x = 1;
    // let y = x;
    // println!("x = {}, y = {}", x, y);

    // let s = String::from("takes");  // create a variable with a string takes
    // takes_ownership(s);  // give ownership to the function
    // println!("{}", s);

    // let val = 1;
    // make_copy(val);
    // println!("{}", val);
    // 
    // let s: String = give_ownership();
    // println!("{}", s);
    // 
    // let s: String = take_and_give(s);
    // println!("{}", s);
    // 
    // if true {
    //     let str1 = s;
    // } else {
    //     let str2 = s;
    // }

    // let mut str1 = String::from("jerok");
    // let mut str2: String;
    // 
    // loop {
    //     str2 = str1;
    // }

    let mut s = String::from("hello");
    change_string(&mut s);
    println!("{}", s);
}

fn change_string(str: &mut String) {
    str.push_str(", world!");
}

fn takes_ownership(s: String) {
    let string = s;
    println!("{}", string);
}

fn make_copy(num: i32) {
    let value = num;
    println!("{}", value);
}

fn give_ownership() -> String {
    "given".to_string()
}

fn take_and_give(string: String) -> String {
    string
}
