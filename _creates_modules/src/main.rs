mod english;  // Showing that this is the module that exists in the current directory;
mod parent {
    pub fn get_by_id() { 
        println!("This is the function inside the modules");
    }

    pub mod child1 {
        // Additional functionality can go here
    }
}

use parent::get_by_id as getfunction;

use english::greetings;

fn main() {
    getfunction(); // Just call the function, no need to println! its result
    let data_comes : String = greetings::greeting::modules_hello();
    print!("{}", data_comes);
}
