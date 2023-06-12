/*
 * Struct std::vec::Vec
 */

use std::ops::{Index, IndexMut};

fn main() {
    let v: Vec<i32> = Vec::new();
    let v: Vec<i32> = vec![];
    let v = vec![1, 2, 3, 4, 5];
    let v = vec![0; 10];  // ten zeroes

    let mut v = vec![1, 2];
    v.push(3);

    let mut v = vec![1];
    let one = v.pop().unwrap();
    println!("{:?}", one);
    let two = v.pop();
    println!("{:?}", two);

    let mut v = vec![1, 2, 3];
    let three = v[2];
    v[1] = v[1] + 5;

    // *container.index(index)
    let three = *v.index(2);  // same: let three = v[2];
    println!("{}", three);
    
    let mut v = vec!["one".to_string(), "two".to_string(), "three".to_string()];  // move occurs because value has type `String`, which does not implement the `Copy` trait
    let value = v.index(2);
    println!("{}", value);
    
    let mut v = vec!["one", "two", "three"];
    let value = *v.index(2);
    println!("{}", value);

    // Index, IndexMut
    let mut balance = Balance {
        right: Weight::Kilogram(2.5),
        left: Weight::Pound(1.5),
    };

    // In this case, `balance[Side::Right]` is sugar for
    // `*balance.index(Side::Right)`, since we are only *reading*
    // `balance[Side::Right]`, not writing it.
    assert_eq!(balance[Side::Right], Weight::Kilogram(2.5));

    // However, in this case `balance[Side::Left]` is sugar for
    // `*balance.index_mut(Side::Left)`, since we are writing
    // `balance[Side::Left]`.
    balance[Side::Left] = Weight::Kilogram(3.0);

    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);

    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], 1);

    assert_eq!(vec.pop(), Some(2));
    assert_eq!(vec.len(), 1);
    
    vec[0] = 7;
    assert_eq!(vec[0], 7);
    
    vec.extend([1, 2, 3]);

    for x in &vec {
        println!("{x}");
    }
    assert_eq!(vec, [7, 1, 2, 3]);

    let mut vec1 = vec![1, 2, 3];
    vec1.push(4);
    let vec2 = Vec::from([1, 2, 3, 4]);
    assert_eq!(vec1, vec2);

    let vec = vec![0; 5];
    assert_eq!(vec, [0, 0, 0, 0, 0]);
    
    // The following is equivalent, but potentially slower:
    let mut vec = Vec::with_capacity(5);
    vec.resize(5, 0);
    assert_eq!(vec, [0, 0, 0, 0, 0]);

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        // Prints 3, 2, 1
        println!("{top}");
    }
}

#[derive(Debug)]
enum Side {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
enum Weight {
    Kilogram(f32),
    Pound(f32),
}

struct Balance {
    pub left: Weight,
    pub right: Weight,
}

impl Index<Side> for Balance {
    type Output = Weight;

    fn index(&self, index: Side) -> &Self::Output {
        println!("Accessing {index:?}-side of balance immutably");
        match index {
            Side::Left => &self.left,
            Side::Right => &self.right,
        }
    }
}

impl IndexMut<Side> for Balance {
    fn index_mut(&mut self, index: Side) -> &mut Self::Output {
        println!("Accessing {index:?}-side of balance mutably");
        match index {
            Side::Left => &mut self.left,
            Side::Right => &mut self.right,
        }
    }
}
