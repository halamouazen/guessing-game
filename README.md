# guessing-game
A robust, command-line interface (CLI) game built with Rust.
The program generates a secret random number between 1 and 100, and the player must guess it using logic and feedback provided by the system. 

How to run the game: 
1. Ensure you have Rust and cargo installed
Install them using rustup: 
curl https://sh.rustup.rs -sSf | sh 
2. clone the repository and navigate to the project directory: 
git clone https://github.com/<your-username>/guessing-game.git
cd guessing-game
3. Run the game: 
cargo run


Game Logic Flow
1. Generate a secret number between 1 and 100.
2. loop indefinitely to request user input. 
3. compare the guess using Ordering enum: 
less ->"Too small"
Greater -> "Too big" 
Equal -> "You win!"(Triggersa break to exit the loop).
5. Track the total number of attempts and display it upon victory. 
6. End the game if the player guesses correctly or runs out of attempts. 

Tests: 
I have implemented automated unit tests to ensure the game handles input parsing correctly and doesn't crash on invalid input.
To verify the project's status 
cargo test 
Example tests include: 
parsing valid numbers like "42/n" 
Handling invalid inputs such as "apple" 
Verifying comparison logic between guesses and the secret number

Git and Github workflow:
-git init: initializes a new git repository.
-echo "target/\nCargo.lock" > .gitignore: Creates a file named 
.gitignore to tell Git which files to ignore.
git add .: Adds all files to project directory (.=everything)
git commit -m: creates a commit which is like a save point
git brunch add-tests : Creates new brunch called add-test. 
git push -u origin main: pushes the local file to the repository. 

Learning Goals: 
This project helped practice: 
-Basic Rust syntax and control flow 
-Using the rand and colored crates
-cargo commands (run, build, test)
-writing and running unit tests
-Git and Github workflows



