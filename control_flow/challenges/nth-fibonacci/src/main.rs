fn main() {
    for i in 1.. {
        println!("the {i}<ordinal suffix> fibonacci number is:\n{}\n", fib(i));
        println!("the {i}<ordinal suffix> factorial number is:\n{}\n", factorial(i));
    }
}

fn fib(n: u128) -> u128 {
    // if n < 1 {
    //     println!("please enter a number greater than 0");
    //     return 0;
    // }

    if n <= 1 {
        return 1
    }

    fib(n - 1) + fib(n - 2)
}

fn factorial(n: u128) -> u128 {
    let mut sum = 1;
    
    for i in 1..=n {
        sum *= i;
    }

    // dbg!(sum);
    // dbg!(n);
    // dbg!(1..=n);

    return sum;
}