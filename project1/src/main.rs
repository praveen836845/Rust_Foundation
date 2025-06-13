// generics struct storage which store the data

mod struct_module {
    pub mod datastorage;
}
use struct_module::datastorage::GenericsSet;

// We need to import the trait because:
// 1. Rust allows multiple traits to have methods with the same name
// 2. The compiler needs to know which trait's implementation to use
// 3. Even though we implemented the trait for GenericsSet, we still need to bring the trait into scope
use crate::struct_module::datastorage::Structbehaviour;

fn main() {
    println!("Project related to the user and How it gonna works ,");
    // creating the Struct, with vector values ,
    let instance_struct = GenericsSet{
        set1: vec![1, 2, 3, 4, 5],
        set2: vec![4.0, 5.0, 5.0, 6.2],
    };
    println!("{:?}", instance_struct.set1);
    
    // This works because we imported both GenericsSet and Structbehaviour
    instance_struct.subset();
    
    // If we remove the Structbehaviour import, this would fail to compile
    // because the compiler wouldn't know which trait's subset() method to use
}
