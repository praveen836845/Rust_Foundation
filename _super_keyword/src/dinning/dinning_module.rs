pub fn dining_function() {
    println!("Function in dining module");
    

    // To access lib.rs functions, you need to go up TWO levels:
    super::super::print_receipt();
    
    // Or use the absolute path:
    crate::print_receipt();
}
