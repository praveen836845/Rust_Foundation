pub mod greeting {
    pub fn modules_hello(name: String, email: String) -> Details {
        // Storing the String the Struct, then trying to access the modules,
        return Details {
            name: name,
            email: email,
        };
    }
    #[derive(Debug)]
    pub struct Details {
        pub name: String,
        pub email: String,
    }
}
