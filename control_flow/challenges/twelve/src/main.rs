fn main() {
    let ordinals = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let lyrics = ["a partridge in a pear tree", "two turtle doves", "three french hens", "four calling birds", "five gold rings", "six geese a-laying", "seven swans a-swimming", "eight maids a-milking", "nine ladies dancing", "ten lords a-leaping", "eleven pipers piping", "twelve drummers drumming"];

    prelude(ordinals[0]);
    println!("{}", lyrics[0]);

    // nested for loop; outer counts i from 2 up to 12 (as first verse is slightly different), inner counts j down from i to 1
    for i in 2..=12 {
        prelude(ordinals[i]);
        for j in i..=2 {
            println!("{}", lyrics[j]);
        }
        println!("and {}", lyrics[0]);
    }
}

fn prelude(ordinal: &str) {
    println!("on the {} day of christmas, my true love gave to me", ordinal);
}
