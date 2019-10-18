mod functions;

use std::io;
use std::io::Write;

fn main() {
    loop {
        let mut user_choice = String::new();
        print!("\
Welcome to my calculator, do you follow me on GitHub?
1 - Addition
2 - Subtration
3 - Multiplication
4 - Division
5 - Xor operation
6 - Or operation
7 - Left shift
8 - Right shift
9 - Exit
-> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut user_choice)
            .expect("Failed to get the user input");

        println!("");

        let user_choice: i32 = user_choice.trim().parse()
            .expect("Please, type a number");

        match user_choice {
            1 => functions::add(),
            2 => functions::subtration(),
            3 => functions::multiplication(),
            4 => functions::division(),
            5 => functions::bitwise::xor(),
            6 => functions::bitwise::or(),
            7 => functions::bitwise::left_shift(),
            8 => functions::bitwise::right_shift(),
            9 => break,
            _ => println!("Type a number between 1 and 9")
        }
    }
}
