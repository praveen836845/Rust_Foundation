// src/main.rs
use _super_keyword::place_order;

fn main() {
    place_order("Pasta Carbonara");

    // can i acces in the mod.rs as super keywords : NO ----------
     pub fn accessbile_main(){
        println!("Hey i'm inside the main.rs function and accessible to the mod.rs by using super keyword.")
     }
}