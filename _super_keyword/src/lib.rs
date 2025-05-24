use dinning::dinning_module; // This is basically doing that , scoping the dinning module to the lib   

// src/lib.rs
pub mod kitchen; // This declares and exposes the kitchen module
// pub mod dinning; // Uncomment this line

pub mod dinning {
    pub mod dinning_module;
}


// Function in the root module
pub fn print_receipt() {
    println!("Printing customer receipt from the main module");
    
}

// Public function to place orders
pub fn place_order(dish: &str) {
    println!("Order received for: {}", dish);
    kitchen::prepare_meal(dish);
    
    // caling the submodules function :: using the dinning::dinning_modules
     dinning::dinning_module::dining_function();
     
     // calling the another function inside the dinning module
     dinning::dinning_module::another_dinning_function();


}