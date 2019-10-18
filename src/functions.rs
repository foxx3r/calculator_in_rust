use std::io;
use std::io::Write;

pub mod bitwise {
    use std::io;
    use std::io::Write;

    pub fn xor() {
        let mut first_expr = String::new();
        let mut second_expr = String::new();
    
        print!("Type the first number -> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut first_expr)
            .expect("Failed to get the user input");
        println!("");

        let first_expr: i128 = first_expr.trim().parse()
            .expect("Please, type a number");

        print!("Type the second number -> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut second_expr)
            .expect("Failed to get the user input");

        println!("");

        let second_expr: i128 = second_expr.trim().parse()
            .expect("Please, type a number");

        println!("The result is: {}", first_expr ^ second_expr);
    }

    pub fn or() {
        let mut first_expr = String::new();
        let mut second_expr = String::new();

        print!("Type the first number -> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut first_expr)
            .expect("Failed to get the user input");

        println!("");

        let first_expr: i128 = first_expr.trim().parse()
            .expect("Please, type a number");

        print!("Type the second number -> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut second_expr)
            .expect("Failed to get the user input");

        println!("");

        let second_expr: i128 = second_expr.trim().parse()
            .expect("Please, type a number");

        println!("The result is: {}", first_expr | second_expr);
    }

    pub fn left_shift() {
        let mut first_expr = String::new();
        let mut second_expr = String::new();

        print!("Type the first number -> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut first_expr)
            .expect("Failed to get the user input");

        println!("");

        let first_expr: i128 = first_expr.trim().parse()
            .expect("Please, type a number");

        print!("Type the second number -> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut second_expr)
            .expect("Failed to get the user input");

        println!("");

        let second_expr: i128 = second_expr.trim().parse()
            .expect("Please, type a number");

        println!("The result is: {}", first_expr << second_expr);
    }

    pub fn right_shift() {
        let mut first_expr = String::new();
        let mut second_expr = String::new();

        print!("Type the first number -> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut first_expr)
            .expect("Failed to get the user input");

        println!("");

        let first_expr: i128 = first_expr.trim().parse()
            .expect("Please, type a number");

        print!("Type the second number -> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut second_expr)
            .expect("Failed to get the user input");

        println!("");

        let second_expr: i128 = second_expr.trim().parse()
            .expect("Please, type a number");

        println!("The result is: {}", first_expr >> second_expr);
    }
}

pub fn add() {
    let mut first_expr = String::new();
    let mut second_expr = String::new();

    print!("Type the first number -> ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut first_expr)
        .expect("Failed to get the user input");

    println!("");

    let first_expr: i128 = first_expr.trim().parse()
        .expect("Please, type a number");

    print!("Type the second number -> ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut second_expr)
        .expect("Failed to get the user input");

    println!("");

    let second_expr: i128 = second_expr.trim().parse()
        .expect("Please, type a number");

    println!("The result is: {}", first_expr + second_expr);
}

pub fn subtration() {
    let mut first_expr = String::new();
    let mut second_expr = String::new();

    print!("Type the first number -> ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut first_expr)
        .expect("Failed to get the user input");

    println!("");

    let first_expr: i128 = first_expr.trim().parse()
        .expect("Please, type a number");

    print!("Type the second number -> ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut second_expr)
        .expect("Failed to get the user input");

    println!("");

    let second_expr: i128 = second_expr.trim().parse()
        .expect("Please, type a number");

    println!("The result is: {}", first_expr - second_expr);
}

pub fn multiplication() {
    let mut first_expr = String::new();
    let mut second_expr = String::new();

    print!("Type the first number -> ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut first_expr)
        .expect("Failed to get the user input");

    println!("");

    let first_expr: i128 = first_expr.trim().parse()
        .expect("Please, type a number");

    print!("Type the second number -> ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut second_expr)
        .expect("Failed to get the user input");

    println!("");

    let second_expr: i128 = second_expr.trim().parse()
        .expect("Please, type a number");

    println!("The result is: {}", first_expr * second_expr);
}

pub fn division() {
    let mut first_expr = String::new();
    let mut second_expr = String::new();

    print!("Type the first number -> ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut first_expr)
        .expect("Failed to get the user input");

    println!("");

    let first_expr: i128 = first_expr.trim().parse()
        .expect("Please, type a number");

    print!("Type the second number -> ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut second_expr)
        .expect("Failed to get the user input");

    println!("");

    let second_expr: i128 = second_expr.trim().parse()
        .expect("Please, type a number");

    println!("The result is: {}", first_expr / second_expr);
}
