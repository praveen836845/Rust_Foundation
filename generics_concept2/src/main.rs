 use std::fmt::Display;
fn generics_reverse<T: Display>(vector: &mut Vec<T>) {
    while let Some(last) = vector.pop() {
        //                     [3]
        println!("{}", last);
    }


}

// Reverse the string vectors
fn main() {
    // let mut Str_vector: Vec<String> = vec!["Hello".to_string(), "World".to_string(), "Rust".to_string(), "Generics".to_string()];
    let mut str_vector : Vec<&str> = vec!["Hello", "World", "Rust", "Generics"];
    generics_reverse(&mut str_vector);

    let mut int_vector : Vec<i32> = vec![1, 2, 3, 4, 5];
    generics_reverse(&mut int_vector);
}

