// Declare the `fn_mod` module and expose its `fn_module` submodule to the compiler.
// This tells Rust to look for a file at: src/fn_mod/fn_module.rs
mod fn_mod {
    pub mod fn_module;
}


// Bring the `fn_module` submodule into scope so we can access its items without full path qualification.
use fn_mod::fn_module;
fn main() {
    // Call the `frq_list` function defined in the `fn_module` submodule.
    
    let  vectorlist: Vec<fn_module::UserDetails> = vec![
        fn_module::UserDetails {
            id: 1,
            name: String::from("Praveen"),
            age: 18,
            description: String::from("This is the smartest person"),
        },
        fn_module::UserDetails {
            id: 2,
            name: String::from("Ravi"),
            age: 22,
            description: String::from("Loves learning Rust"),
        },
      fn_module::UserDetails {
            id: 3,
            name: String::from("Anjali"),
            age: 20,
            description: String::from("UI/UX designer and coder"),
        },
    ];   


    fn_module::process_list(&vectorlist);

}
