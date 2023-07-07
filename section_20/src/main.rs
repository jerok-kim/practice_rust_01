fn factorial(n: i32) -> i32 {
    if n < 2 {
        return n;
    }

    n * factorial(n - 1)
}

fn fibonacci(n: i32) -> i32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}

fn palindrome(input: &str) -> bool {
    let mut result = true;
    let mut index = 0;
    let vec: Vec<char> = input.chars().collect();
    for e in input.clone().chars().rev() {
        if vec[index] != e {
            result = false;
            break;
        }
        index += 1;
    }
    result
}

fn hanoi(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }
    2 * hanoi(n - 1) + 1
}

fn main() {
    // println!("{}", factorial(5));
    // println!("{}", fibonacci(6));
    // println!("{}", palindrome("hello"));
    // println!("{}", palindrome("123454321"));
    println!("{}", hanoi(3));
}
