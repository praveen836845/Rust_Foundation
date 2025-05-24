pub fn dining_function() {
    println!("**********************Function inside the Dinning_modules as it's a Direct function inside");
    

    // To access lib.rs functions, you need to go up TWO levels:
    super::super::print_receipt();
    
    // Or use the absolute path:
    crate::print_receipt();
}

pub fn another_dinning_function(){
    println!("**********************Function inside the Dinning_modules as it's a Direct function inside");
}