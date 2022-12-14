use std::io::{self, Write};
use std::process::Command;

fn main() {


    // Super janky way of clearing the output
    Command::new("clear").status().expect("Error using 'Clear' Command??\n");

    let limit: i32 = i32_input("enter a limit");

    let lines: i32 = i32_input("how many lines do you want me to print");

    fizz_buzz(limit, lines);

}


fn i32_input(msg: &str) -> i32 {

    println!("\n{msg}");
    println!("+------------------------------+");

    
    loop {

        // reset / init buffer
        let mut buffer: String = String::new();
        
        // prompt
        io::stdout()
            .write(b"> ").unwrap();

        io::stdout()
            .flush().unwrap();


        io::stdin()
            .read_line(&mut buffer)
            .expect("failed to read line");

        
        let buffer: i32 = match buffer.trim().parse::<i32>() {

            Ok(num) => num,

            Err(_e) => {

                println!("\nPlease enter a valid number!");
                continue;

            }
        };

        return buffer;
    }

}

fn fizz_buzz(limit: i32, lines: i32) {

    let mut iter: i32 = limit - lines;


    while iter <= limit {
        iter += 1;

        if iter < 0 {continue;}
        
        let mut output: String = String::new();

        if iter % 3 == 0{output.push_str("Fizz")}
        if iter % 5 == 0{output.push_str("Buzz")}

        println!("{iter}: {output}")

    }

}
