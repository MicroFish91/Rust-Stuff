use std::io::{ stdin };
use std::cmp::Ordering;

fn main() {
    let secret_number = 32;

    loop {
        let mut input: String = String::new();
        // let secret_number = rand::random::<u8>();

        println!("Please guess a number: ");

        stdin().read_line(&mut input).expect("Could not read value.");
        // let input = input.trim().parse::<i32>().expect("Invalid number.");
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        println!("Secret number is: {}", secret_number);
        println!("Your input was: {}", input);

        // if secret_number == input {
        //     println!("It's a match!!");
        // } else if secret_number > input {
        //     println!("You guessed too low.");
        // } else {
        //     println!("You guessed too high.");
        // }

        match input.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is less."),
            Ordering::Equal => {
                println!("Your guess matched. Gg!");
                break;
            },
            Ordering::Greater => println!("Your guess was greater."),
        }
    }
}
