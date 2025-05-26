fn main() {
    // Case 1: Ownership transfer (String is not Copy)
    let s1 = String::from("Hello");
    let s2 = s1; // s1's ownership is moved to s2
    // println!("s1: {}", s1); // ❌ ERROR: s1 is no longer valid here
    println!("s2: {}", s2); // ✅ OK

    // Case 2: Copy trait (i32 implements Copy)
    let x = 42;
    let y = x; // x is copied to y

    println!("x: {}", x); // ✅ OK, x is still accessible
    println!("y: {}", y); // ✅ OK

// Example of .push_str 

let mut s11= String::from("foo");
let s22 = "bar";
s1.push_str(s22);
println!("s2 is {s2}");
println!("S1 is {s1}");  // As the ownership is not transfer from this process....





}






