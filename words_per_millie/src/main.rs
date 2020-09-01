extern crate time;

use std::time::Duration;

fn main() {
    let mut x:u128 = 0u128;
    let time = std::time::Instant::now();
    while time.elapsed().as_millis() < 1 {
        println!("words");
        x += 1;
    };
    println!("Rust prints {} 'words' in {}ms!", x, time.elapsed().as_millis());
} 
