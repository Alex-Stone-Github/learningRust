extern crate rand;

use std::io;
use rand::thread_rng;
use rand::Rng;


fn main() {
    println!("Hi there everyone this is like my third time using rust, and I am going to try and make a game with it. Wish me luck!");
    let mut line = String::new();



    loop {
        println!("Guess a number between 0 and 5");
        let _ = io::stdin().read_line(&mut line); // References = pretty cool
        let mut rng = thread_rng();
        let random_num: i32 = rng.gen_range(0, 5);
        line.pop(); // Get rid of the last newline char
        let guess_num: i32 = line.parse::<i32>().unwrap(); // Convert String to num
        println!("random: {}  + guess: {}", random_num, guess_num);
        if random_num == guess_num {
            println!("You guessed correctly!");
        } else {
            println!("Sadly, you guessed wrong, want to try again?");
        }
    }
}
