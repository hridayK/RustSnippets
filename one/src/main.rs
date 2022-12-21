use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let x: u32 = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number!");
    println!("Please input your guessed number:");
    
    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("failed to read the line!");

    println!("You guessed: {guess}");

    let guess_num: u32 = guess.trim().parse().expect("Please type a number!");
    
    println!("The number generated was: {x}\nand you guessed: {guess}");
    match guess_num.cmp(&x){
        Ordering::Less => println!("{guess_num} < {x}"),
        Ordering::Greater => println!("{guess_num} > {x}"),
        Ordering::Equal => println!("{guess_num} = {x}"),
    }
}
