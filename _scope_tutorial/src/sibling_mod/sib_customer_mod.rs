// This is the first approach declaring the nested and newer Style to access the function and making it in scope...
/* 
pub mod customer {
    // Two ways to Handle this module call inside the module one 
    // use key using the abosulate path of the function and make in the scope...
    use crate::front_house::frontofhouse::front_of_house::hosting;
 
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

*/


// ******************************************* Second way of doing ***********************************************

pub mod customer {
  // Two ways to Handle this module call inside the module one 
  // use key using the abosulate path of the function and make in the scope...
use crate::front_house::front_of_house::hosting;
  pub fn eat_at_restaurant() {
    // crate::front_house::frontofhouse::front_of_house::hosting::add_to_waitlist(); 
   hosting::add_to_waitlist();
  }
}