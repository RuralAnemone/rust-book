use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("fun number guessin game");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut guesses: u32 = 0;

    loop {
        println!("guess a number!:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("uh oh couldn't read the line :0");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("that's  not a number, numbnuts. try again");
                continue;
            }
        };

        println!("your guess was: {}", guess);

        // I actually kinda like the fact you can't use var++ or ++var cus it mitigates confusion
        guesses += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small (that's what she said)"),
            Ordering::Greater => println!("too big! (that ain't what she said)"),
            Ordering::Equal => {
                println!("you win in {guesses} guesses");
                break;
            }
        }
    }
}
