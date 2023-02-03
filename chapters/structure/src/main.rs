struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//Tuple structs
struct Colour(i32, i32, i32); //basically a named Tuple
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height 
    }
}

fn main() {
    structs_intro();
    
    areastruct();
}

fn structs_intro() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"), 
        ..user1 // user1 now invalid because of username being copied. Username is String, has drop trait.
    };
    
    let user3 = build_user(String::from("anotherone@example.com"), String::from("username"));
    let black = Colour(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn areastruct() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    println!("Rect1 is {:#?}", rect1);
    println!("The area of the rectangle is {}", rect1.area());

    let rect2 = Rectangle {
        width: 10, 
        height: 40,
    };
    
    let rect3 = Rectangle {
        width: 60, 
        height: 45,
    };

    let square1 = Rectangle::square(30);
    
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold square1? {}", rect1.can_hold(&square1));
}
