extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Guess what number:");

    let mut pred = String::new();
    io::stdin()
        .read_line(&mut pred)
        .expect("Failed to readline");

    println!("Typed:{}",pred);

    // Convert string to num(uint32)
    let pred_u :u32 = pred.trim().parse().expect("Convert str to num failed.");

    /* Generate random number */
    let ans = rand::thread_rng().gen_range(1,101); // Generate number with range 1-101
    println!("ans:{}",ans);

    /* Do process for each cases */
    match pred_u.cmp(&ans){
        Ordering::Less => println!("Too low"),
        Ordering::Greater => println!("Too high"),
        Ordering::Equal => println!("That's it"),
    }
}
