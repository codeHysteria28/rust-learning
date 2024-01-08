fn main() {                 // s is not valid here, it's not yet declared
    let mut s = String::from("hello"); // s is valid from this point forward
    // do stuff with s

    s.push_str(", world !"); // push_str() appends a literal to a String
    println!("{}", s); // this will print `hello, world!`

    // double free error = memory safety bug = when both s1 and s2 go out of the scop, they will both try to free the same memory
    let s1 = String::from("hello");
    // let s2 = s1; // to ensure memory safety, after this line, Rust considers s1 as no longer valid 
    
    // deep copy of the heap data
    let s2 = s1.clone();

    // s1 = hello, s2 = hello
    println!("s1 = {}, s2 = {}", s1, s2);

    //? Stack-only data copy

    // fixed known value, both of equal size pushed to the stack
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // ? Ownership and functions

    let s3 = String::from("hello"); //* s3 comes into scope

    takes_ownership(s3); //* s3's value moves into the function...
                                    //*... and so is no longer valid here

    let j = 5; //* j comes into scope

    makes_copy(j); //* x would move into the function,
                                //* but i32 is Copy, so it's okay to still
                                //* use x afterward
}// this scope is now over, and s is no longer valid 

fn takes_ownership(some_string: String){ // some_string comes into scope
    println!("{}", some_string);
}   // Here, some_string goes out of scope and `drop` is called. The backing
    // memory is freed.

fn makes_copy(some_integer: i32){ // some_string comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.