extern crate rand;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn greet(){
    println!("Guess the number!!")
}

fn main() {
    greet();

    let secret_number = rand::thread_rng().gen_range(1,101);

    loop{
        println!("Plse input your guess : ");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Plse enter a valid number");
                continue
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
