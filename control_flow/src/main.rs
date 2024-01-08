fn main() {
    //* if expressions

    let number = 5;

    if number < 5 {
        println!("condition was true");
    }else if number == 5 {
        println!("the number equals 5");
    }else {
        println!("condition was false");
    }

    let condition = true;
    let number2 = if condition { 5 } else { 6 };
    println!("The value of number is: {number2}");

    //* loops
    //? while, for, loop
    //? loop will execute over and over again or until explicitly stop
    // loop {
    //     println!("again!")
    // }

    // returning values from Loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}");

    // loop labels to disambiguate between multiple loops
    let mut count = 0;
    'counting_top: loop {
        println!("count = {count}");

        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_top;
            }

            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // looping through the collection with for
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the values is: {element}");
    }

    // using rev() : reverses the iterator direction - range; countdown loop
    for number in (1..4).rev() {
        println!("{number}");
    }

    println!("LIFTOFF !!!");
}
