
🦀 Guessing Game


A robust, command-line interface (CLI) number guessing game built with Rust.
The program generates a secret random number between 1 and 100, and the player must guess it using logic and feedback provided by the system.

⸻

🚀 How to Run the Game
	1.	Ensure you have Rust and Cargo installed
Install them using rustup:
curl https://sh.rustup.rs -sSf | sh

Clone the repository and navigate to the project directory:

git clone https://github.com/<your-username>/guessing-game.git
cd guessing-game

Run the game:

cargo run

Cargo automatically installs dependencies like rand and colored the first time you run the project.

🧠 Game Logic Flow
	1.	Generate a secret random number between 1 and 100 using the rand crate.
	2.	Loop to continuously request user input.
	3.	Validate the input:
	•	If the input is invalid (letters or symbols), the program displays a warning and asks again.
	4.	Compare the guess to the secret number using the Ordering enum:
	•	Less → “📉 Too small!”
	•	Greater → “📈 Too big!”
	•	Equal → “🎉 You win!” and the loop exits.
	5.	Track the total number of attempts and display it upon victory.
	6.	End the game if the player guesses correctly or runs out of attempts.

Testing

I have implemented automated unit tests to ensure the game handles input parsing correctly and doesn’t crash on invalid input.

To verify the project’s status:
Cargo test

Example tests include:
	•	Parsing valid numbers like "42\n"
	•	Handling invalid inputs such as "apple"
	•	Verifying comparison logic between guesses and the secret number

⚙️ Error Handling

Instead of crashing on invalid input (like typing letters),
the game uses a match expression to catch parsing errors and display:

⚠️ “That’s not a number! Please try again.”

This ensures a smooth and user-friendly experience.

🧰 Git and GitHub Workflow

Typical workflow used for this project:

git init
echo "target/\nCargo.lock" > .gitignore
git add .
git commit -m "Initial commit: implemented guessing game"
git branch add-tests
git checkout add-tests
git push -u origin main

You can open a Pull Request (PR) on GitHub to merge your test branch into main.

🎯 Learning Goals

This project helped practice and reinforce:
	•	Basic Rust syntax and control flow
	•	Using the rand and colored crates
	•	Running and managing projects with Cargo (run, build, test)
	•	Writing and executing unit tests
	•	Using Git and GitHub for version control and collaboration
