use std::io;
fn main() {
// test 1
    christmas();
// test 2
    rust_fibonacci(15);
// test 3
    fibonacci(15);
    println!("Fahrenheit 37.0 to Celsius {}", convert_temperatures(37.0));
    loop_example_4();
    loop_example_3();
    loop_example_2();
    loop_example_1();
    variable_example();
    shadowing_example();
    tuple_example();

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    array_example(index);

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let sum = plus_one(five());
    println!("The value of sum is: {sum}");

    control_flow(15);
}

fn christmas() {
    {
        let gifts = [
            "a Partridge in a Pear Tree",
            "two Turtle Doves",
            "three French Hens",
            "four Calling Birds",
            "five Gold Rings",
            "six Geese a-Laying",
            "seven Swans a-Swimming",
            "eight Maids a-Milking",
            "nine Ladies Dancing",
            "ten Lords a-Leaping",
            "eleven Pipers Piping",
            "twelve Drummers Drumming",
        ];

        for day in 1..=12 {
            print!(
                "On the {} day of Christmas, my true love gave to me, ",
                day_of_christmas(day)
            );
            for gift in (0..(day as usize)).rev() {
                if gift == 0 {
                    print!("{}", gifts[gift]);
                } else {
                    print!(", {}", gifts[gift]);
                }
            }
            println!();
        }
    }
}
fn rust_fibonacci(num: u64) {
    println!("Fibonacci sequence up to {num}");
    for i in (0..num) {
        println!("{}", fibonacci2(i));
    }
}

fn day_of_christmas(day: u32) -> &'static str {
    match day {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => "",
    }
}

fn fibonacci2(num: u64) -> u64 {
    match num {
        0 => 0,
        1 => 1,
        num => fibonacci2(num - 1) + fibonacci2(num - 2),
    }
}
fn fibonacci(num: u64) {
    let mut per: u64 = 0;
    let mut next: u64 = 1;
    // 0 1
    for i in (1..num) {
        print!(" {} ", next + per);
        next += per;
        per = next - per;
    }
}
fn loop_example_1() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn loop_example_2() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");

        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }
}

fn loop_example_3() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("done");
}

fn convert_temperatures(temp: f32) -> f32 {
    (temp - 32.0) * 5.0 / 9.0
}

fn loop_example_4() {
    let a = [1, 2, 3, 4, 5];

    let mut index = 0;

    while index < a.len() {
        println!("The value is: {}", a[index]);
        index += 1;
    }
}

fn loop_example_5() {
    let a = [1, 2, 3, 4, 5];
    for element in a {
        println!("The value is: {}", element);
    }
}

fn variable_example() {
    // var
    let mut x: i32 = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 5;
}

fn shadowing_example() {
    // shadowing
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");
}

fn tuple_example() {
    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (a, b, c) = tup;

    println!("The value of a is: {a}");
    println!("The value of b is: {b}");
    println!("The value of c is: {c}");

    // let five_hundred = tup.0;
    // let six_point_four = tup.1;
    // let one = tup.2;
}

fn array_example(index: String) {
    // Array
    let array = [1, 2, 3, 4, 5];

    // let months= ["January", "February", "March", "April", "May", "June", "July",
    // "August", "September", "October", "November", "December"];

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = array[index];

    println!("The value of the element at {index} is: {element}");
}

fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1
}

fn control_flow(num: i32) {
    if num % 2 == 0 {
        println!("Number is even");
    } else {
        println!("Number is old");
    }

    let number = if num % 2 == 0 { 1 } else { 0 };

    println!("The value of number is: {number}");

    let mut message = String::new();

    if num % 5 == 0 {
        message = message + "FIZZ";
    }

    
    if num % 3 == 0 {
        message = message + "BUZZ";
    }

    if message.is_empty() {
        println!("the number is {num}");
    } else {
        println!("{}", message);
    }
}
