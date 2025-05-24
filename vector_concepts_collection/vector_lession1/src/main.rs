mod vector_storing; // telling the rust that this file modules is exist in our crate...
// use vector_storing; // Making the functions accessible in this module...
/*fn main() {
let mut value :i8 = 4;
stored_fn_vector(&mut value);
println!("Changes in the value {}", value);
} */


/* 
fn main() {
    let mut value :i8 = 4 ; 
    stored_fn_vector(&mut value);
    println!("Changes in the value {}", value);
} */

fn main(){
    let mut array:[i8;4] = [1,2,3,4];
    vector_storing::stored_fn_vector(&mut array);
    println!("Changes in the array {:#?}", array);
    vector_storing::printing_vector();

    println!("Changed Array : *******************");
     vector_storing::chaning_array(&mut array);
     println!("Modified Array: {:#?}", array);


}