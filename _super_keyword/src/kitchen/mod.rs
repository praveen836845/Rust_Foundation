// src/kitchen/mod.rs
pub fn prepare_meal(dish: &str) {
    println!("Kitchen preparing: {}", dish);
    check_ingredients();
    cook_dish();
    
    // Here's where we use super to access the parent module
    super::print_receipt();
    // Without super, we would need to use:
    // crate::print_receipt();

}

fn check_ingredients() {
    println!("Checking ingredients in stock");
}

fn cook_dish() {
    println!("Cooking dish to perfection");
}