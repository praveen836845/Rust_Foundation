use std::fmt;
use serde::Serialize;


// Define a concrete struct (non-generic in this case)
// This represents user data with specific field types
// Demonstrates basic struct usage before we generalize it
#[derive(Debug ,Serialize , Clone)]
pub struct UserDetails {
    pub id: i8,
    pub name: String, 
    pub age: i8, 
    pub description: String,
}

// Define a trait (interface) called Details
// Traits declare behavior that types can implement
// This uses concrete types (i8) but could be made generic
trait Details{
    // Method signature: takes a reference to self and an id,
    // returns a UserDetails struct
    // The &[UserDetails] parameter suggests we'll search a collection
    fn get_details_by_id(&self, id: i8, list: &[UserDetails]) -> UserDetails;
    
    // Another method that modifies state, returning a status message
    // fn update_details_by_id(&self, id: i8) -> String;
}

// Implement the Details trait for UserDetails
// Shows how to provide concrete behavior for a type
impl Details for UserDetails {
    // Actual implementation of get_details_by_id
    // This would search the slice for matching ID
    fn get_details_by_id(&self, id: i8, lists: &[UserDetails]) -> UserDetails {
   
       for list in lists.iter(){
         if list.id == id {
             return list.clone();
         }
       }
        UserDetails {
            id: 0,
            name: String::new(),
            age: 0,
            description: String::new(),
        }
    }

}



// A generic function that takes any type T
// Demonstrates simple generics usage
// The <T> declares a generic type parameter
pub fn process_list<T: serde::Serialize>(list : &[T]) {
    let instance = UserDetails {
        id: 0,
        name: "".to_string(),
        age: 0,
        description: "".to_string(),
    };
    let data = instance.get_details_by_id(2, list); // throw an error , why ???

    let json = serde_json::to_string_pretty(&list).unwrap();
    println!("{json}");

}

