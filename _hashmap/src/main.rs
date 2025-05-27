
mod hshmap_mod {
    #[path = "../hshmap_mod/hshmap.rs"]
    pub mod hshmap;
}


use hshmap_mod::hshmap::hmap_mod;
fn main() {
    println!("HashMap in this crate root");
    // let mut Hashmap =  HashMap::new();
    // calling the function;
    hmap_mod::hashmap_function();
}