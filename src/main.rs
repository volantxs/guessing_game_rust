use std::io;
use rand::Rng; 
use std::cmp::Ordering; 

fn main() {
    println!("Guess the number game!");
        let secret_num: u32 = rand::thread_rng().gen_range(1..=100);

    loop{
        println!("Input your guess:");
        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess) 
        .expect("Failed to read line"); //if Err
        let guess : u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
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
