use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::{stdin};

fn main() {
    

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut guess = String::new();
    let mut input = String::new();
    let mut online: bool = true;

    while online
    {
        println!("Guess the number!");

        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess was Too small!"),
            Ordering::Greater => println!("Your guess was Too big!"),
            Ordering::Equal => println!("You guessed Correct!"),
        }

        println!("Do you wish to keep guessing?");

        stdin().read_line(&mut input).expect("Didnt enter a correct string");

        if input == "Y"
        {
            online = true;
        }else if input == "N"
        {
            online = false;
        }
        else
        {
            println!("Error: Incorrect respose! Will proceed to close application.");
            online = false;
        }
    }
}