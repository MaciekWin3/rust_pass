use rand::Rng;

fn main() {
    println!("Welcome to rust_pass!");
    println!("1. Generate new password");
    println!("2. Read password");
    println!("Press q to exit");

    //let mut option = String::new();

    println!("{{}}");

    for i in 1..16 {
        let number = rand::thread_rng().gen_range(0..characters.len());
        password += &characters[number - 1..number];
    }

    println!("{}", password);
    //io::stdin()
    //    .read_line(&mut option)
    //    .expect("Failed to read the line");
}

fn generate_password(length: i32, include_uppercase: bool,
    include_special_characters: bool, include_numbers: bool) -> String {
    let mut password = String::new();
    let lowercase_letters = "abcdefghijklmnopqrstuvwxyz";
    let uppercase_letters = lowercase_letters.to_uppercase();
    let numbers = "0123456789";
    let special_characters = "`!@#$%^&*()<>.,?;':[]{}";
    let password_builder = String::new();

    if include_uppercase {
        password_builder += &uppercase_letters;
    }

    if include_numbers {
        password_builder += &numbers;
    }

    if include_special_characters {
        password_builder += &special_characters;
    }

    for i in 0..length {

    }
    //let characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    password
}