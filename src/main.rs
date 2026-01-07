use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
  println!("Guess a number rn!");

  let secret_number = rand::thread_rng().gen_range(1..=100);

  loop {
      println!("Input the guessed number:");

  let mut guess = String::new();

  io::stdin()
         .read_line(&mut guess)
        .expect("Failed to read line");

         println!("You guessed: {guess}");

         let guess:u32 = match guess.trim()
                              .parse() {
                              Ok(num) => num,
                              Err(_) => continue
                              };

         match guess.cmp(&secret_number) {

            Ordering::Less => println! ("Too small"),
            Ordering::Greater =>println!("Too  Big"),
            Ordering::Equal => {
                println!("You got the guess right!🎊");
                break;
         }
            }

  }

  

}