fn main() {
    // Here is how you would define and use a calculate_length 
    // function that has a reference to an object as a parameter instead of taking ownership of the value:

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // mutable reference
    let mut s2 = String::from("hello");
    change(&mut s2);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // here, s goes out of the scope. But because it does not have ownership of what
  // it referes to, it's not dropped.

fn change(some_string: &mut String){
    some_string.push_str(", world");
}
