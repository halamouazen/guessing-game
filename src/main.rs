use std::io; 
use std::cmp::Ordering;
use rand::Rng;
use colored::*; 
fn main() {
    let secret_number: u32 =
     rand::thread_rng().gen_range(1..=100); //choose a random number between 1-100.
    let mut guesses_left= 7;
     let mut total_attempts= 0; 
      println!("{}", "=== WELCOME TO THE RUST GUESSING GAME ===!".bright_cyan().bold()); 

    loop {
        if guesses_left == 0 {
            println!("\n{}", "GAME OVER!".red().bold());
            println!("The secret number was: {}", secret_number.to_string().yellow());
            break;
        }
        println!("\nGuesses remaining: {}" , 
        guesses_left.to_string() .cyan());
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse()
     {
    Ok(num) => num, 
    Err(_) => {
           println!("{}", "That's not a number!".yellow());
           continue; 
           }
        };
          total_attempts += 1; 
          match guess.cmp(&secret_number) { 
            Ordering::Less => {
            println!("{}", "Too small!".red());
             guesses_left -= 1; 
        }
        Ordering::Greater => {
            println!("{}", "Too big!".red());

               guesses_left -= 1;
        }
        Ordering::Equal => { 
            println!("\n{}","YOU GOT IT!".green().bold());
            println!("It took you{}attempts.",
            total_attempts.to_string().green());
            break; 
 }
}
}
}



#[cfg(test)] 
mod tests {
    use super::*;
 #[test]
    fn test_parse_input() {
        let input = "42\n";
        let parsed: Result<u32, _> =
        input.trim().parse();
        assert_eq!(parsed.unwrap(), 42);
    }
    #[test]
fn test_invalid_input() {
    let input = "apple" ;
    let parsed: Result<u32, _> =
    input.trim().parse();
    assert!(parsed.is_err());
}
}

    
    
   