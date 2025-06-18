#[derive(Debug)] // Added Debug derive for the struct
struct DataStorage<T> { // Rust convention uses PascalCase for type names
    values: i32,
    generics_data: T, // snake_case for field names
}

trait MethodSignature {
    // Default method that will call another function with self as parameter
    fn default_method(&self) {
        println!("Calling default_method from MethodSignature");
        self.called_function(); // Call the implemented method
    }

    // This should be implemented by types, not have a default impl
    fn called_function(&self);
}

// Implementing the trait for DataStorage<T>  
// When implementing a trait for a generic struct, the impl must account for generics:

impl<T: std::fmt::Debug> MethodSignature for DataStorage<T> {
    fn called_function(&self) {
        println!(
            "Hi, I'm being called by the default method!\nData: {:?}",
            self
        );
    }
}

fn main() {
    println!("When the Implementing the generics Struct then impl block also must be generics");
    
    let instance = DataStorage {
        values: 23,
        generics_data: "Hello_generics_key".to_string(),
    };
    
    instance.default_method();
}