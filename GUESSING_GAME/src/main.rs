use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;
fn main() {
    let mut life = 3;
    println!("Guess the number!");
    let secret_number= rand::thread_rng().gen_range(1..101);
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        println!("You guessed: {}", guess);
    
       let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            continue
        },
       };

       match  guess.cmp(&secret_number) {
        Ordering::Less => {
            life = life-1;
            println!("{}","Too low!".red());
        },
        Ordering::Greater => {
            life = life-1;
            println!("{}","Too high!".red());
        },
        Ordering::Equal => {
            println!("{}","You win!".green());
            break;
    },
       }


       if life < 1 {
        println!("{}{}","you loose the number was ".red(),secret_number.to_string().green());
        break;
       }


    }
    
}