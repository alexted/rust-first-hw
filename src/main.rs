extern crate core;

use std::io;

use crate::christmas_song::christmas_song::sing_song;
use crate::fibonacci_generator::fibonacci_generator::number_generator;
use crate::temperature_converter::temperature_converter::converter;

mod christmas_song;
mod fibonacci_generator;
mod temperature_converter;

fn main() {
    // Играем в игры
    loop {
        println!("
Select the program:
    1. Temperature conversion between Fahrenheit and Celsius
    2. Generating the n-th Fibonacci number
    3. The Christmas song \"The Twelve Days of Christmas\"
Input the number:");

        let mut game_number = String::new();

        io::stdin().read_line(&mut game_number).expect("Failed to read line");
        let game_number: u8 = match game_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("It's not a valid integer!");
                continue;
            }
        };

        match game_number {
            1 => converter(),
            2 => number_generator(),
            3 => sing_song(),
            _ => println!("It's not a valid integer!")
        }
    }
}