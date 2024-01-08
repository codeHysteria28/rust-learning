fn main() {
    another_function(5);

    another_function2(5, 'h');

    let y = five();
    println!("The value of y is: {y}");

    let i = plus_one(5);
    println!("The value of i is: {i}");

}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn another_function2(value: i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}")
}

//* Statements - are instructions that perform some action and do not return a value
//* Expressions - evaluate to a resultant value

// ? Function with Return values
// ! we do not name return values, but must declare return type

fn five() -> i32 {
    5
}

fn plus_one(i: i32) -> i32 {
    i + 1
}