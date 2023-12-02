use std::time::SystemTime;

use crate::days::day_2::day_2;

pub mod days;


fn main() {
    let now = SystemTime::now();

    println!("{}", day_2(2));

    println!("duration: {}", now.elapsed().unwrap().as_micros())
}