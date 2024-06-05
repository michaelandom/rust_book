enum  IpAddrkind {
    v4(i8,i8,i8,i8),
    v6    
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32,i32,i32)
}

impl  Message {
    
    fn some_function() {
        println!("Let's Get Rust");
    }
}
// struct IpAddr {
//     kind: IpAddrkind,
//     address: String
// }

struct IpAddr {
    ip: IpAddrkind,
}



#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


fn main() {
    example_1();
    example_2();
    example_3();
    example_4();
    example_5();
    example_6();
    }
    
    fn example_1() {
    let four = IpAddrkind::v4(127,0,0,1);
    let six = IpAddrkind::v6;
    let localhost = IpAddr {
        ip: IpAddrkind::v4(127,0,0,1),
    };
    }



    fn example_2() {
        let some_number= Some(5);
        let some_string= Some("string");
        let absent_number: Option<i32> = None;
    }

    fn example_3(){

        let x: i8=5;
        let y = Some(5);

        let sum = x + y.unwrap_or(0);
    }

    fn example_4() {
        value_in_cents(Coin::Quarter(UsState::Alaska));
    }

    fn example_5(){
      let five  = plus_one(Some(4));
      let six  = plus_one(five);
      let none  = plus_one(None);
      println!("n is {:?}", none);
    }


    fn example_6() {
        let some_value =Some(3);
        match some_value {
            Some(n) => println!("3"),
            _ => ()
        }



        if let Some(3) = some_value {
            println!("There");
        }

    }

    fn plus_one(num: Option<i32>) -> Option<i32>{
        match  num {
            Some(n) => Some(n+1),
            _ => None,
        }
        
    }
fn value_in_cents(coin: Coin) -> u8 {

    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(us_state) => {
            println!("State is {:#?}",us_state);
            25
        },
    }
}
fn route(ip_kind: IpAddrkind) {
    
}
