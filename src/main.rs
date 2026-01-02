use std::io;

fn main() {
  println!("Guess a number!");
  println!("Input the guessed number:");

  let mut guess = String::new();

  io::stdin()
         .read_line(&mut guess)
        .expect("Failed to read line");

         println!("You guessed: {guess}");

}