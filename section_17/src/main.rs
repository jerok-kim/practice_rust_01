// use std::pin::Pin;
// use std::task::{Context, Poll};

use std::io;
use async_std::fs::File;
use async_std::io::ReadExt;
use async_std::task;

async fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}

fn main() {
    let task = task::spawn(async {
        let result = read_file("read.txt").await;
        match result {
            Ok(k) => println!("{}", k),
            Err(e) => println!("Error reading from file!: {}", e),
        }
    });
    
    println!("Task has started!");
    task::block_on(task);
    println!("Stopped task!");
}

// trait Future {
//     type Output;
//     fn poll(self: Pin<&mut self>, cx: &mut Context) -> Poll<Self::Output>;
//     
//     poll::ready, poll::pending
// }
