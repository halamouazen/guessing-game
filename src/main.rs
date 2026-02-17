use std::io; 
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Welcome to the Guessing Game!"); 
    
     let secret_number =
     rand::thread_rng().gen_range(1..=100); //choose a random number between 1-100.
     let mut attempts= 0; 

    loop {
        println!("please input your guess(1-100):");
        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    //Handle non-integer
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
                continue;      
        }
    }; 
    attempts += 1;

     match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win! It took you {} attempts.", attempts); 
                    break;
         }    } 
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

    
    
   