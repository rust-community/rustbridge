#![feature(rand)]

extern crate rand;

use rand::Rng;

fn main() {
    println!("Hello, world!");
    let mut rng = rand::thread_rng();
    if rng.gen() { // random bool
        println!("i32: {}, u32: {}", rng.gen::<i32>(), rng.gen::<u32>())
    }
}
