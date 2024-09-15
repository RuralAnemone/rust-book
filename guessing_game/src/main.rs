use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("fun number guessin game");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("the secret nubmer is {secret_number}");

    println!("guess a number!:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("uh oh couldn't read the line :0");

    let guess: u32 = guess.trim().parse().expect("that's  not a number, numbnuts. try again");

    println!("your guess was: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("too small (that's what she said)"),
        Ordering::Greater => println!("too big! (that ain't what she said)"),
        Ordering::Equal => println!("you win I guess"),
    }
}