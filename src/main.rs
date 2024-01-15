use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main(){
    println!("Guess a number(1-100).");
    let secret_number = rand::thread_rng().gen_range(1..=100);
   loop{
        println!("Enter the number you guessed : ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line.");
        println!("You guessed {guess}");
        let guess :u32 = match guess.trim().parse() {
            Ok(num)=>num,
            Err(_)=>continue,
        };
        match guess.cmp(&secret_number){
            Ordering::Less=>println!("Too small!!"),
            Ordering::Greater=>println!("Too big!!"),
            Ordering::Equal=>{
                println!("You win!!");
                break;
            }
        }   
    }
}