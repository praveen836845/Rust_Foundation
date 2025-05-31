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
pub trait Details<T> {
    // Method signature: takes a reference to self and an id,
    // returns Option<T> since the item might not be found
    // The &[T] parameter suggests we'll search a collection
    fn get_details_by_id(&self, id: i8, list: &[T]) -> Option<T>;
    
    // Another method that modifies state, returning a status message
    // fn update_details_by_id(&self, id: i8) -> String;
}


// Implement the Details trait for UserDetails
// Shows how to provide concrete behavior for a type
impl Details<UserDetails> for UserDetails {
    // Actual implementation of get_details_by_id
    // This would search the slice for matching ID
    fn get_details_by_id(&self, id: i8, lists: &[UserDetails]) -> Option<UserDetails> {
        for list in lists.iter() {
            if list.id == id {
                return Some(list.clone());
            }
        }
        None // Return None if not found instead of creating a dummy UserDetails
    }
}




// A generic function that takes any type T
// Demonstrates simple generics usage
// The <T> declares a generic type parameter
pub fn process_list<T: serde::Serialize + Clone + std::fmt::Debug>(list: &[T]) 
where 
    UserDetails: Details<T>
{
    let instance = UserDetails {
        id: 0,
        name: "".to_string(),
        age: 0,
        description: "".to_string(),
    };
    
    // Handle the Option result properly
    match instance.get_details_by_id(2, list) {
        Some(data) => println!("Found item: {:?}", data),
        None => println!("Item with id 2 not found"),
    }
    
    let json = serde_json::to_string_pretty(&list).unwrap();
    println!("{json}");
}
