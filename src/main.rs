use rand::Rng;

fn main() {
    println!("Welcome to rust_pass!");
    println!("1. Generate new password");
    println!("2. Read password");
    println!("Press q to exit");

    //let mut option = String::new();

    //io::stdin()
    //    .read_line(&mut option)
    //    .expect("Failed to read the line");

    println!("Generated Password: {}", generate_password(16, true, true, true));
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