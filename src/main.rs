use rand::{Rng, thread_rng};

fn main() {
    let mut rng = thread_rng();
    let random_byte = rng.gen::<u8>();

    println!("Guess the random number: ");

    loop {
        // Get user input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        // parse input to u8
        let input: u8 = input.trim().parse().unwrap();

        if input == random_byte {
            println!("You guessed correctly!");
            break;
        } 
        else if input > random_byte {
            println!("Too high!");
        }
        else {
            println!("Too low!");
        }
    }
    println!("The random byte was: {}", random_byte);
}