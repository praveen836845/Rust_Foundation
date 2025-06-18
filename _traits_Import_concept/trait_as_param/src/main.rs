#[derive(Debug)] // Added Debug derive for the struct
struct DataStorage<T> {
    // Rust convention uses PascalCase for type names
    values: i32,
    genericsdata: T, // snake_case for field names
}

trait MethodSignature {
    // Default method that will call another function with self as parameter
    fn default_method(&self) {
        println!("Called the Default Method and Now calling the Sibling functions in the trait");
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

fn trait_as_parameters(value: impl MethodSignature) -> impl MethodSignature {
    println!(
        "I'm the Function with papermerter as Trait  on shield type : values"
    );
    value.called_function();
    return DataStorage {
        values: 32,
        genericsdata: "Stringvalue".to_string(),
    };
}

fn main() {
    println!("Creating instance and Calling the Default Method that are implmented in the Trait");

    let instance = DataStorage {
        values: 23,
        genericsdata: "Hello_generics_key".to_string(),
    };

    instance.default_method();
    trait_as_parameters(instance);
}
