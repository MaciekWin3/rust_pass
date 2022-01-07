use rand::Rng;
use std::io;
use std::process;

fn main() {
    println!("Welcome to rust_pass!");
    println!("1. Generate new password");
    println!("2. Generate and save password");
    println!("3. Show password");
    println!("3. Read password");
    println!("4. Update password!");
    println!("Press q to exit");

    let mut option = String::new();

    io::stdin().read_line(&mut option).unwrap_or_else(|err|{
        println!("Error: {}", err);
        process::exit(1);
    });

    println!("{}", option);

    match option.as_str().trim() {
        "1" => {
            let password = generate_password(16, true, true, true);
            println!("Generated password: {}", password);
        },
        _ => {
            println!("Działa");
            //panic!("Error!");
        }
    }

    //println!("Generated Password: {}", generate_password(16, true, true, true));
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