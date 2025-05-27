pub mod hmap_mod {
    use std::collections::HashMap;
    pub fn hashmap_function() {
        println!("Inside the function Modules");

        let var1 = String::from("First Variable");
        let var2 = String::from("second Variable");
        let mut hashmap_variable = HashMap::new();

        // Add some key-value pairs
        hashmap_variable.insert(String::from("key1"), &var1);
        hashmap_variable.insert(String::from("key2"), &var2);
        // hashmap_variable.insert(String::from("Key3"), String::from("3"));  // mismatch type error in the Hashmap;

        println!("HashMap contents: {:?}", hashmap_variable);

        let check_value = hashmap_variable.entry(String::from("key1"));

        println!("{check_value:?}"); // This is the Actual way to print the way the Hashmap;
                                     // println!("{:p}", check_value);  // @@@@@@ Not working on it yet.....
        println!("{var1}"); // Throw an error as Hashmap owns the ownership ,

        println!("***************Printing the library********************");
        for (key, value) in &hashmap_variable {
            println!("{key} : {value}");
        }

        // Updating the value based on the Old value.....
        let text = "Hello world wonderful world";

        let mut mapped = HashMap::new();
        for data in text.split_whitespace() {
            let count = mapped.entry(data).or_insert(0); // returning the mutatble reference of the value;
            println!("{:p}", count);
            *count += 1;

        }

        println!("{mapped:?}");
    }
}
