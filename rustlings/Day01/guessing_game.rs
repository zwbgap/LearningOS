/*
 * @Author: ZAM
 * @Date: 2023-10-01 14:36:46
 * @LastEditors: zam
 * @LastEditTime: 2023-10-01 14:38:16
 * @Description: file content
 * @FilePath: \LearningOS\rustlings\Day01\guessing_game.rs
 */


/*fn main() {
    println!("Hello, world!");
}*/
use std::io;
use rand::Rng;
use std::cmp::Ordering;
/**
 * @description: guessing_game
 * @return {*}
 */
fn guessing_game() {
    println!("Guess the number!");
    //generate secret number
    let secret_number = rand::thread_rng().gen_range(1,101);
    //shadowing or delete secret_number
    //println!("The secret number is: {}", secret_number);
    loop {
        println!("Please input your guess:");
        //read guess
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        //change guess type
        let guess: u32 = match guess.trim().parse() {
            //Handle illegal input
            Ok(num) => num,
            Err(_) => continue,
        };
            //.expect("Please type a number!");
        println!("You guessed: {}", guess);
        //compare guess and secret_number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
fn main() {
   guessing_game();
}