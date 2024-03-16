use std::io;                /* lib for input output */
use rand::Rng;              /* random generetor for range 0..=100*/
use std::cmp::Ordering;     /* three statements: Less, Greater, Equal*/

fn main() {

    println!("Guesse the number: ");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess: ");
    
    let mut i: u32 = 0;
    let mut won: bool = false;
    loop {
        
        if i == 6 { break; }
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { println!("You win!"); won = true; break; }
        }
        i += 1;
    }
    if !won {
        println!("The secret number is: {secret_number}");
    }

}
