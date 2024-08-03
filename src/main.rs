pub mod dsa;

use std::io;
use std::io::Write;
use rand::Rng;
use dsa::fibonacci::f;

const PROMPT_STR: &str = "Please input your number: ";

fn main(){
    let list = f(100);
    for x in list.iter(){
        println!("{}", x);
    }
}

fn guess_number(){
    println!("Guess the number! :)");
    let random_value = rand::thread_rng().gen_range(0..10);
    loop {
        let input = prompt();
        match input.parse::<i32>(){
            Ok(n) => {
                if n != random_value{
                    println!("You guessed wrong!");
                    continue;
                }
                println!("You guessed correct! Answer was: {0}", random_value);
                return;
            },
            Err(_) => {
                println!("OI! Ya knobhead, enter a number!");
                continue;
            }
        }
    }
}

fn prompt() -> String {
    print!("{PROMPT_STR}");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    return input.trim().to_string();
}