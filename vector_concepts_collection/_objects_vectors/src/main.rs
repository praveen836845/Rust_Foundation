#[warn(dead_code)]
    #[derive(Debug)]
    struct PersonAbility {
        name: String,
        age: i8,
        cognetive_ability: String,
    }



fn main() {

    // check that how can insert this struct into the vector
    let object1 = PersonAbility {
        name: String::from("Praveen"),
        age: 12,
        cognetive_ability: String::from("logical_ability : level2")
    };

    let object2 = PersonAbility {
        name: String::from("Praveen"),
        age: 12,
        cognetive_ability: String::from("logical_ability : level2")
    };

    let mut vector: Vec<PersonAbility> = Vec::new();
    vector.push(object1);
    vector.push(object2);

    // println!("Printing the Object : {:#?}  {:#?}" , vector[0] , vector[1]);
    // println!("{:#?}", vector);
   /*for data in vector {
    println!("Details of the person : {:#?} ",  vector);  // as the Ownership is trnasfer to the data varible, 
   } */

   for data in vector {
    println!("Details of the person : {:#?} ",  data);  // as the Ownership is trnasfer to the data varible, 
   }

}
