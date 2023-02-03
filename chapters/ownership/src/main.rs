fn main() {
    ownership();
    reference_borrowing();
    slice();
}

fn ownership() {
    let mut s = String::from("hello");
    let nostr = ", world!";

    s.push_str(nostr);

    println!("{}", s);
    let mut x = 5;
    let y = x;
    println!("{}, {}", x, y);
    x += 1;
    println!("{}, {}", x, y);
}

fn reference_borrowing() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);    
}

fn slice() {
    let mut s = String::from("hellow world");
    let _word = first_word(&s);

    s.clear();
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}