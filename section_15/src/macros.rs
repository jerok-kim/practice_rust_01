use macros::debug_print;

macro_rules! gcd {
    ($a: expr, $b: expr) => {
        {
            let mut m = $a;
            let mut n = $b;
            
            while m != 0 {
                if m < n {
                    let t = m;
                    m = n;
                    n = t;
                }
                m = m % n;
            }
            
            n
        }
    };
}

macro_rules! average {
    ( $(,)* ) => {{
        0.0
    }};
    
    ( $($val: expr), + $(,)* ) => {{
        let count = 0_usize $(+ { let _ = stringify!($val); 1})*;
        let sum = 0.0 $(+ $val as f64)*;
        sum / count as f64
    }};
}

#[debug_print]
fn main() {
    println!("{}", gcd!(14, 15));
    println!("{}", gcd!(20, 10));

    println!("Hello, world!");
    println!("{}", average!());
    println!("{}", average!(1.0, 2.0, 3.0));
    println!("{}", average!(1, 2, 3, 4, 5));
}
