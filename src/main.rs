use rand::Rng;
use std::io::{stdout, Write};
use std::io;
use std::process;
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand,
    Result,
    event
};

fn main() {
    loop {
        display_menu();

        let mut option = String::new();

        io::stdin().read_line(&mut option).unwrap_or_else(|err|{
            println!("Error: {}", err);
            process::exit(1);
        });


        //println!("{}", option);

        match option.as_str().trim() {
            "1" => {
                let password = generate_password(16, true, true, true);
                println!("Generated password: {}", password);
            },
            "2" => {
                todo!();
            }
            "3" => {
                todo!();
            }
            "4" => {
                todo!();
            },
            "5" => {
                todo!();
            },
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

fn display_menu() -> Result<()> {

    stdout()
        .execute(SetForegroundColor(Color::Blue))?
        .execute(Print(
            "Welcome to rust_pass!\n\
            --------------------------------- \n\
            1. Generate new password \n\
            2. Generate and save password \n\
            3. Show password \n\
            4. Read password \n\
            5. Update password \n\
            --------------------------------- \n\
            Press q to quit \n\
            --------------------------------- \n\
            Choose option: "))?
        .execute(ResetColor)?;

    Ok(())
}

fn generate_password(length: i32, include_uppercase: bool,
    include_special_characters: bool, include_numbers: bool) -> String {

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