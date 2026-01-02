use std::io;

fn main (){
    println!("guess a number");
    println!("input the number");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to receive input");

        println!(" you guessed: {guess}")
}