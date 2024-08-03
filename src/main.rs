pub mod dsa;

use std::io;
use std::io::Write;
use dsa::arrays::binary_search;
use rand::Rng;
use dsa::fibonacci::f;
use dsa::arrays::bubble_sort;
use dsa::arrays::selection_sort;
use dsa::arrays::linear_search;
use dsa::arrays::SearchData;

const PROMPT_STR: &str = "Please input your number: ";

fn main(){
    let mut vec = vec![7, 12, 9, 11, 3];
    let mut ordered = vec![17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70,83,84,85,86,87,88,89,90,91,92,93,94,95,96,97,98,99,100,101,102,103,104,105,106,107,108,109,110,111,112,113,114,115,121,122,123,124,125,126,127,128,129,130,131,140,141,142,143,144,145,146,147,148,149,150,151,152,153,154,155,156,157,158,159,160,161,162,163,164,165,166,167,168,169,170,171,172,173,174,175,176,177,178,179,180,181,182,183,184,185,186,187,188,198,199,200,201,202,203,204,205,206,207,208,209,210,211,212,213,214,215,216,226,227,228,229,230,231,232,238,239,240,241,242,243,244,245,246,247,248,249];
    let data = dsa::arrays::binary_search(&mut ordered, 1345);
    println!("Index: {0}, Operations: {1} | {2}", data.index, data.operations, ordered[data.index as usize]);
    return;

    let mut list = f(100);
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