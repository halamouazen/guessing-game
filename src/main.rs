use std::io; 
use std::cmp::Ordering;
use rand::Rng;
use colored::*; 
fn parse_guess(input: &str) -> Result<u32, String> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Err("Input was empty".to_string());
    }
    let number: u32 = trimmed
    .parse()
    .map_err(|_| format!("'{}' is not a valid number", trimmed))?;
if !(1..=100).contains(&number) {
    return Err(format!("{} is out of range", number));
}
Ok(number)
}

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

    let guess  = match parse_guess(&guess) {
     Ok(num) => num,
    Err(msg) => {
        println!("{}", msg);
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
    fn parse_guess_valid_number() {
        let result = parse_guess ("42\n");
        assert_eq!(result.unwrap(), 42);
    }
    #[test]
fn parse_guess_trims_spaces() {
    let result = parse_guess("   7   ");
    assert_eq!(result.unwrap(), 7);
}

#[test]
fn parse_guess_rejects_empty_input() {
    let result = parse_guess("   ");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Input was empty");
}

#[test]
fn parse_guess_rejects_non_number() {
    let result = parse_guess("apple");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not a valid number"));
}
#[test]
fn parse_guess_rejects_zero() {
    let result = parse_guess("0");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("out of range"));
}
#[test]
fn parse_guess_rejects_out_of_range_high() {
    let result = parse_guess("101");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("out of range"));
}
}
    
   