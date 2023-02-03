fn main() {
    another_function(5);
    print_labeled_measurements(5, 'h');
    if_five();
    if_expression();
    break_loop_name();
    while_loop();
    loop_collection();

    let fib_num = fibonnaci(0, 1, 7);
    println!("Fibnum is {fib_num}")
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}")
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn if_five() {
    let x = plus_one(4);
    println!("The value of x is: {x}");

    if x > 5 {
        println!("plus_one() worked");
    } else {
        println!("plus_one() worked but you passed 4 or less lol");
    }
}

fn if_expression() {    
    let condition = true;
    let con_num = if condition {5} else {6};

    println!("The value of con_num is: {con_num}");
}

fn break_loop_name() {    
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 8 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    let mut num = 3;
    while num != 0 {
        println!("{num}");

        num -= 1;
    }
    println!("LIFTOFFF BITCH ;D")
}

fn loop_collection() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    println!("While loop");
    while index < 5 {
        println!("The value is {}", a[index]);

        index += 1;
    };

    println!("For loop");
    for element in a {
        println!("The value is {element}")
    };

    println!("For loop range");
    for num in (1..4).rev() {
        println!("{num}");
    }
    println!("LIFTOFF!");
}

fn fibonnaci(elem1: i32, elem2: i32, x: i32) -> i32 {
    let mut fib1 = elem1;
    let mut fib2 = elem2;
    let mut fib_num = 0;
    if x == 1 {
        return fib1; 
    } else if x == 2 {
        return fib2;
    } else {
        let mut num = 0;
        while num < x-2 {
            fib_num = fib1 + fib2;
            fib1 = fib2;
            fib2 = fib_num;
            num += 1; 
        }
        fib_num
    }
}