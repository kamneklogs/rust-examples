use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101); //Limite inferior inclusivo, superior exclusivo

    //println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess");

        let mut guess = String::new(); // mut permite que guess pueda cambiar de valor

        io::stdin() //Parecido a getLine() en c++
            .read_line(&mut guess) // .read_line() retorna un tipo Result, puede ser Ok or Err, commo Exception types
            .expect("Failed to read line"); //Rompe el programa si el type Result es Err

        println!("You guessed: {}", guess);

        //: u32 entero sin signo de 32 bits, se expecifica para que parse() funcione correctamente
        let guess: u32 = match guess.trim().parse() {
            //Usamos match para capturar alguna excepcion
            Ok(num) => num,
            Err(_) => continue, //Si el parseo falla, se omite la ejecucion del resto del loop y se inicia la siguiente iteracion
        };
        match guess.cmp(&secret_number) {
            // Match -> Semejante a una sentencia switch
            Ordering::Less => println!("Too small!"), //Ordering es un enum, puede tener esas tres variantes
            Ordering::Greater => println!("Too big!"), //Cada opcion es llamada un arms o brazo, tiene un patron y el codigo a ejecutar si el match retorna dicho patron o variante
            Ordering::Equal => {
                //Atencion a las variaciones de las arrow functions
                println!("You win!");
                break; //Curiosa forma de romper el loop
            }
        }
    }
}
