extern crate passwords;

use crossterm::{
    event, execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    ExecutableCommand, Result,
};
use passwords::PasswordGenerator;
use rand::Rng;
use std::io;
use std::io::stdout;
use std::process;

fn main() {
    loop {
        display_menu();

        let mut option = String::new();

        io::stdin().read_line(&mut option).unwrap_or_else(|err| {
            println!("Error: {}", err);
            process::exit(1);
        });

        match option.as_str().trim() {
            "1" => {
                let password = generate_password(16, true, true, true, true, false, true, true);
                println!("Generated password: {}", password);
            }
            "2" => {
                println!("Please provide informations below");
            }
            "3" => {
                todo!();
            }
            "4" => {
                todo!();
            }
            "5" => {
                todo!();
            }
            "q" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid command! Please try again or exit program by pressing q");
            }
        }
    }
}

fn generate_password_user_input() -> Result<()> {
    stdout().execute(Print("Length: "))?;
    Ok(())
}

fn display_menu() -> Result<()> {
    stdout()
        .execute(SetForegroundColor(Color::Magenta))?
        .execute(Print(
            "Welcome to rust_pass!\n\
            --------------------------------- \n\
            1. Quick password generation \n\
            2. Generate password \n\
            3. Show password \n\
            4. Read password \n\
            5. Update password \n\
            --------------------------------- \n\
            Press q to quit \n\
            --------------------------------- \n\
            Choose option: ",
        ))?
        .execute(ResetColor)?;

    Ok(())
}

fn generate_password(
    length: usize,
    numbers: bool,
    lowercase_letters: bool,
    uppercase_letters: bool,
    symbols: bool,
    spaces: bool,
    exclude_similar_characters: bool,
    strict: bool,
) -> String {
    let pg = PasswordGenerator {
        length,
        numbers,
        lowercase_letters,
        uppercase_letters,
        symbols,
        spaces,
        exclude_similar_characters,
        strict,
    };
    return pg.generate_one().unwrap();
}

fn generate_password_obsolete(
    length: i32,
    include_uppercase: bool,
    include_special_characters: bool,
    include_numbers: bool,
) -> String {
    let mut password = String::new();
    let mut password_builder = String::from("abcdefghijklmnopqrstuvwxyz");

    let uppercase_letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let numbers = "0123456789";
    let special_characters = "`!@#$%^&*()<>.,?;':[]{}";

    if include_uppercase {
        password_builder += &uppercase_letters;
    }

    if include_numbers {
        password_builder += &numbers;
    }

    if include_special_characters {
        password_builder += &special_characters;
    }

    let mut i = 0;

    while i < length {
        let number = rand::thread_rng().gen_range(0..password_builder.len());
        password += &password_builder[number - 1..number];
        i += 1;
    }
    password
}
