use std::io;

fn main() {
    println!("fun number guessin game");

    println!("guess a number!:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("uh oh couldn't read the line :0");

    println!("your guess was: {}", guess);
}