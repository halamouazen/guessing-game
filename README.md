# guessing-game
A robust, command-line interface (CLI) game built with Rust.
The program generates a secret random number, and the player must guess it using logic and feedback provided by the system. 

How to run the game: 
1. Ensure you have Rust and cargo installed
2. Get the source code onto your machine. 
3. open your terminal in the project folder and run: cargo run

I have implemented automated unit tests to ensure the game 
handles inputs correctly. To verify the project status, 
run:   cargo test

Instead of crashing on invailed input (like typing letters),
The game uses a match expression to catch errors and ask the player to try again. 
The game tracks how many guesses you've made and displays the total upon victory. 
Using the rand crate with a thread local random number 
generator,seeded by the system.

Logic Flow
1. Generate a secret number between 1 and 100.
2. loop indefinitely to request user input. 
3. compare the guess using Ordering enum: 
less ->"Too small"
Greater -> "Too big" 
Equal -> "You win!"(Triggersa break to exit the loop).

