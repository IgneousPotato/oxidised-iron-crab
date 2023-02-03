mod medianmode;
mod piglatin;
mod company; 

use medianmode::{median, mode};
use piglatin::to_pig_latin;
use company::company;

fn main() {
    // TASK 1
    let mut v = vec![24, 7, 5, 12, 7, 7, 32, 343, 6, 92];
    v.sort();
    println!("Integers -> {:?}", v);
    
    let v_med = median(&v);
    println!("Median is {}", v_med);

    let v_mode = mode(&v);
    println!("Mode is {}", v_mode);

    // TASK 2
    let s = String::from("string");
    let s2 = to_pig_latin(&s);
    println!("{} becomes {}", s, s2);

    let s3 = String::from("astring");
    let s4 = to_pig_latin(&s3);
    println!("{} becomes {}", s3, s4);

    // TASK 3
    company()
}
