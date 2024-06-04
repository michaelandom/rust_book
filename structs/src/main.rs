#[derive(Debug)]
struct Rectangle {
    wight: u32,
    hight: u32,
}

impl Rectangle {
    fn array(&self) -> u32 {
        self.wight * self.hight
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.wight > other.wight && self.hight > other.hight
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            wight: size,
            hight: size,
        }
    }
}
fn main() {
    example_1();

    example_2();
}

fn example_1() {
    let rect = Rectangle {
        wight: 20,
        hight: 40,
    };
    println!("Value is {:#?}", rect);
    let rect2 = Rectangle {
        wight: 10,
        hight: 20,
    };

    let rect3 = Rectangle {
        wight: 30,
        hight: 40,
    };

    let rect4 = Rectangle::square(40);

    println!("The Area fo the rectangle {} ", rect.array());
    println!("Can rect hold rect2 {} ", rect.can_hold(&rect2));
    println!("Can rect hold rect3 {} ", rect.can_hold(&rect3));
    println!("Value is {:#?}", rect4);
}

struct User {
    username: String,
    email: String,
    active: bool,
    login_count: u32,
}
fn example_2() {
    let mut user1 = User {
        username: String::from("data_file"),
        email: String::from("data_file@yopmail.com"),
        active: true,
        login_count: 1,
    };

    user1.email= String::from("data_file_updated@yopmail.com");
    user1.username= String::from("data_file_updated");
    user1.active= false;
    user1.login_count= 2;


    let email = String::from("data_file1@yopmail.com");
    let username = String::from("data_file1");
    let user2 = create_with_function(email, username);

    let user3 = User {
        email: String::from("data_file2@yopmail.com"),
        username: String::from("data_file"),
        ..user2
    };

}

fn create_with_function(email: String, username: String) -> User {
    User {
        username,
        email,
        login_count: 1,
        active: true,
    }
}
