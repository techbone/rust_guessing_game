use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

  println!("What is your name ?");

  let mut name = String::new();

  io::stdin()
      .read_line(&mut name)
      .expect("input your name!");

      println!("Migoto!, so your name is {name}");

  println!("Guess a number rn!");

  let secret_number = rand::thread_rng().gen_range(1..=10);

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

            Ordering::Less => println! ("common {name} that's Too small 😂"),
            Ordering::Greater =>println!("leemao {name} that's Too  Big 🤦🏾"),
            Ordering::Equal => {
                println!("You got the guess right {name}, smart ass!🎊");
                break;
         }
            }

  }

  

}