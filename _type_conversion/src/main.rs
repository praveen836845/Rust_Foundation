// fn main() {
//     println!("How old are you?");
//     let age = read_string();
//     println!("You are {age} years old.");
// }

// fn read_string() -> String {
//     let mut input = String::new();
//     std::io::stdin()
//         .read_line(&mut input)
//         .expect("can not read user input");
//     input.trim().to_string()
// }


// Well error Handling way and changes into the type : using FromStr of str modules and called as use ::str::from_str; 
use std::str::FromStr;

fn main() {
    println!("How old are you?");
    let age = read_number();
    println!("You are {age} years old.");
}

fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("can not read user input");
    input.trim().to_string()
}

fn read_number() -> u8 {
    let input = read_string();
    u8::from_str(&input).unwrap_or(1)  // This will not crash your program as it pass the default value to you calle method
}