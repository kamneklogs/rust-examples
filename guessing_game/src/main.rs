use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("{}", secret_number);

    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess) // .read_line() retorna un tipo Result, puede ser Ok or Err, commo Exception types
        .expect("Failed to read line"); //Rompe el programa si el type Result es Err

    println!("You guessed: {}", guess);

    let guess: u32 = guess.trim().parse().expect("Error");

    match guess.cmp(&secret_number) {
        // Match -> Semejante a una sentencia switch
        Ordering::Less => println!("Too small!"), //Ordering es un enum, puede tener esas tres variantes
        Ordering::Greater => println!("Too big!"), //Cada opcion es llamada un arms o brazo, tiene un patron y el codigo a ejecutar si el match retorna dicho patron o variante
        Ordering::Equal => println!("You win!"),
    }
}
