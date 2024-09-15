use std::io;
use rand::Rng;

fn main() {
    println!("fun number guessin game");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("the secret nubmer is {secret_number}");

    println!("guess a number!:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("uh oh couldn't read the line :0");

    println!("your guess was: {}", guess);
}