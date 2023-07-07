macro_rules! op {
    ($x:expr, $y:expr, $op:expr) => {
        match $op {
            1 => $x + $y,
            2 => $x - $y,
            3 => $x * $y,
            4 => $x / $y,
            5 => $x % $y,
            _ => -1
        }
    }
}

fn main() {
    println!("{}", op!(5,2,1)); //should print 7
    println!("{}", op!(5,2,2)); //should print 3
    println!("{}", op!(5,2,3)); //should print 10
    println!("{}", op!(5,2,4)); //should print 2
    println!("{}", op!(5,2,5)); //should print 1
    println!("{}", op!(5,2,6)); //should print -1
}
