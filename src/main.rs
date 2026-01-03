use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
  println!("Guess a number!");

  let secret_number = rand::thread_rng().gen_range(1..=100);

  println!("The secret number is {secret_number}");

  println!("Input the guessed number:");

  let mut guess = String::new();

  io::stdin()
         .read_line(&mut guess)
        .expect("Failed to read line");

         println!("You guessed: {guess}");

         let guess:u32 = guess.trim()
                              .parse()
                              .expect("please type a number");

         match guess.cmp(&secret_number) {

            Ordering::Less => println! ("Too small"),
            Ordering::Equal => println!("You win!🎊"),
            Ordering::Greater => println!("Too  Big")
         }


}