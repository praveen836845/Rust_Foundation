#[derive(Debug)]
struct Numbers {
    a: i32,
    b: i32,
}

use std::fmt;
impl fmt::Display for Numbers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Numbers(a: {}, b: {}, sum: {})",
            self.a,
            self.b,
            self.add_of_numbers(self.a, self.b)
        )
    }
}
#[warn(dead_code)]
trait Other {
    fn traits_impl_sum(&self, n1: i32, n2: i32);
}

impl Numbers {
    fn add_of_numbers(&self, n1: i32, n2: i32) -> i32 {
        println!("{}", n1 + n2);
        println!("{:?}", self);
        return n1 + n2;
    }
}

impl Other for Numbers {
    fn traits_impl_sum(&self, n1: i32, n2: i32) {
        println!("{n1} , {n2}");
    }
}

fn main() {
    println!("This is the tutorial of traits and implementation");
    // first creating the Struct data , randomly in the memory ,
    let n: Numbers = Numbers { a: 1, b: 2 };

    println!("{n}");
    let return_number = n.add_of_numbers(n.a, n.b);

    println!("{return_number}");

    println!("*************trait Implementation");
    n.traits_impl_sum(n.a, n.b);
}
