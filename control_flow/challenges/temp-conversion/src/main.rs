use std::io;

fn main() {
    println!("temperature conversion!");

    let mut selection_from_stdin = String::new();
    let mut selection: u8;

    loop {
        println!("choose from the following list:");
        println!("0) Celsius to Fahrenheit");
        println!("1) Fahrenheit to Celsius");
        
        io::stdin()
            .read_line(&mut selection_from_stdin)
            .expect("uh oh couldn't read the line D:");

        selection = match selection_from_stdin.trim().parse() {
            Ok(s) => s,
            Err(_) => {
                println!();
                println!("please input a valid number");
                println!();
                continue;
            }
        };

        if !(0..=1).contains(&selection) {
            println!("please input a number in the range 0..1");
            continue;
        }

        println!("temperature to convert:");
        break;
    }

    let mut input_from_stdin = String::new();
    let input: f64;

    loop {
        io::stdin()
            .read_line(&mut input_from_stdin)
            .expect("uh oh! couldn't read line D:");

        input = match input_from_stdin.trim().parse() {
            Ok(i) => i,
            Err(_) => {
                println!("please enter a valid number");
                continue;
            }
        };

        break;
    }

    if selection == 0 {
        println!("{:?}", c_to_f(input));
    } else if selection == 1 {
        println!("{:?}", f_to_c(input));
    } else {
        println!("um what");
    }
}

fn f_to_c(input: f64) -> f64 {
    (input - 32f64) * (5f64/9f64)
}

fn c_to_f(input: f64) -> f64 {
    input * (9f64/5f64) + 32f64
}
