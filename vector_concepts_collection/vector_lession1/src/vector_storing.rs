// vector_storing.rs
/*use std::sync::Mutex;

// Thread-safe static vector
static V: Mutex<Vec<i8>> = Mutex::new(Vec::new());

pub fn stored_fn_vector(value: &mut i8) {
    println!("Storing data in Mutex  vector ,If you are using the static type ");
    println!("it will throw an error if you are using :-> static to global variable")
    *value += 2;

    // Lock the mutex to safely access the vector
    let mut v = V.lock().unwrap();
    v.push(*value);
}*/

/* 
pub fn stored_fn_vector(value: &mut i8) {
    println!("This is the Function calling as value {}", value);
    println!("This is the Function calling as memory address {:p}", value);
     let loops = *value;
    let mut vector: Vec<i8> = Vec::new();
    for i in 1..=loops {
        vector.push(*value); // Dereference the value before pushing
    }
    println!("This is the vector {:#?}", vector);
}
    */



pub fn stored_fn_vector(array: &mut [i8; 4]) {
    println!("This is the Function calling as value {:?}", array);
    println!("This is the Function calling as memory address {:p}", array);
     let _loops = *array;
    let mut vector: Vec<i8> = Vec::new();

    for value in _loops.iter(){
        vector.push(*value);
    }
    println!("This is the vector {:#?}", vector);
}


// let discussin the Rules of the by passing the array to the function;
pub fn printing_vector(){

    // let arrays : &[i8] = &[1,2,4,5];  // slice of array and also immutable references
    let mut v = vec![1,2,3,4];
    // println!("Array is printing {:#?}", v);
    // let ref_initial_elem = &v[0];// storing the reference of it.... and move this to the down line 54

    v.push(9);

   println!("Printing Array: {:#?}", v); 
//    println!("Pringint arrays reference , {ref_initial_elem}")
}



pub fn chaning_array(array: &mut [i8; 4]) {
    // chaning the Array: 
    for i in array {
       *i = *i+10;
    }  
  
  }