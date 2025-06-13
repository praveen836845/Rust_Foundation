pub struct GenericsSet<T, U> {
    pub set1 : Vec<T> , 
    pub set2 : Vec<U>
}


// defining the behaviour on type 
pub trait Structbehaviour<T,U>{
    fn union(&self);
    fn relation(&self);
    fn subset(&self) -> Vec<Vec<T>>;
}

impl <T,U> Structbehaviour<T,U>for GenericsSet<T , U>  {
  
   fn union(&self){}
   fn relation(&self) {
   }

    fn subset(&self)-> Vec<Vec<T>> {
    
      let emptyvector:Vec<Vec<T>>    = Vec::new();
      println!("Check the Compiler is now using");
      return emptyvector;
   }
    
}