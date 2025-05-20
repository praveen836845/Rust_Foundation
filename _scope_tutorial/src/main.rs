/*
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Added to waitlist!");
        }
    }
}

mod customer {
    // Two ways to Handle this module call inside the module one 
    // use key using the abosulate path of the function and make in the scope...
  use  crate::front_of_house::hosting;
 
    pub fn eat_at_restaurant() {
    //    super::front_of_house::hosting::add_to_waitlist();  // by calling this one too Second way to remove the use keyword  from that one and use this one 
    //    println!("Here is the Super functions called {}", super); // This will throw error why i cannot print the super keyword....
        hosting::add_to_waitlist();
    }
}

fn main() {
    customer::eat_at_restaurant();
}
 */

// ****************************************************The above is the example 1****************************************************************************

// mod front_house {    // This tells Rust and links to the file and use by any other module by using the crate:: abosolute path full path,,
//     pub mod frontofhouse;
// }        

// mod sibling_mod {
//     pub mod sib_customer_mod;
// }

// // Update the import to use the correct path
// use sibling_mod::sib_customer_mod::customer;

// fn main(){
//     // calling the functions and using the statement to get into the scope..
//     customer::eat_at_restaurant();
// }




// ************************************************Second Example**********************************************
#[path = "front_house/frontofhouse.rs"]
mod front_house;
mod sibling_mod {
    pub mod sib_customer_mod;
}

// Update the import to use the correct path
use sibling_mod::sib_customer_mod::customer;

fn main(){
    // calling the functions and using the statement to get into the scope..
    customer::eat_at_restaurant();
}