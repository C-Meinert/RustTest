use rand::Rng;
use std::cmp::Ordering;
use std::io;

const MAX_POINTS: u32 = 100_000;

fn main() {
    // guessing_game();
    variables_mutability();
}

fn variables_mutability() {
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value fo x is: {}", x);

    // let x = 5;
    // let x = x + 1;
    // let x = x * 2;
    // println!("The value fo x is: {}", x);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("The value fo spaces is: {}", spaces);
}

fn guessing_game() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..101);
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

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
