mod test;
mod testfolder;

use testfolder::ayooo2;
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    test::ayooo();
    ayooo2::ayooo2();

    let s1 = String::from("Hello");
    let s2 = String::from(", ZA WARUDO");
    // let s3 = s1.clone() + &s2;
    let s3 = format!("{s1}{s2}");
//    println!("{}", s3);
    println!("{} + {} = {}", s1, s2, s3);

    let mut scores: HashMap<String, &str> = HashMap::new();
     
    scores.insert(String::from("Yellow"), "10");
    scores.insert(String::from("Purple"), "540");
    scores.entry(String::from("Yellow")).or_insert("20");
    
//     let team_name = String::from("Purple");
//     let score = scores.get(&team_name).copied().unwrap_or("0");
    println!("{:?}", scores);
}
