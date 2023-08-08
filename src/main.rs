use std::io;
// io - input/output library 
// std - standard library
use rand::Rng; //rng - random number generators
use std::cmp::Ordering; //cmp - compares  
//Ordering type is another enum and has the variants Less, Greater, and Equal

fn main() {
     

    println!("Guess the number game!");
        let secret_num: u32 = rand::thread_rng().gen_range(1..=100); //inclusive on the lower and upper bounds

    loop{
        println!("Input your guess:");
        let mut guess = String::new();
        // empty string type - growable, UTF-8 encoded bit of text
        io::stdin()
        .read_line(&mut guess) //append that into a string (without overwriting its contents)
        // returns a Result value -->Result’s variants are Ok and Err
        .expect("Failed to read line"); //if Err


// to handle non number bheaviour of the game
        let guess : u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
//  The underscore, _, is a catchall value;
// we’re saying we want to match all Err values, no matter what information they have inside them.


// assumed input to be  anumber always
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        // to convert string to a integer or u32 type
        //Rust allows us to shadow the previous value of guess with a new one.
        // trim - method,  eliminate any whitespace at the beginning and end,
        //        \n --> represents the newline
        // guesss : '5\n'  --> trim eliminates \n
        // parse- converts string into another type
        println!("You guessed: {}", guess);



        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too Small! , Please try again:"),
            Ordering::Greater => println!("Too Large!, Please try again:"),
            Ordering::Equal => {
                println!("Winner Winner Chicken Dinner! Secret number is {} ", secret_num); 
                break; 
             }


            }


        }

}