fn main() {
    let number = 3;

    if number < 5 {
        println!("Condition was true")
    } else {
        println!("Condition was false");
    }

    // Wrong condition, expects a bool. Does not interpret int value as true/false like c/c++
    // if number {
    //     println!("Number was there");
    // }

    if number != 0 {
        println!("Number was something bigger than zero");
    }

    let num = 6;

    if num % 4 == 0 {
        println!("Number is divisible by 4")
    } else if num % 3 == 0 {
        println!("Number is divisible by 3");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let mut condition =  false;

    // This is cool, like a readable ternary line
                    // if true do left else do right
    let val = if condition {5} else {6};

    println!("The value of val is: {val}");


    // Wrong because val is i32 and expects both conditions to be such
    // condition = true;
    // val = if condition {5} else {"six"};
    // println!("The value of val is: {val}");

    // infinitely prints again until a break;
    // loop {
    //     println!("Again");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter; // need to return counter from loop in order for result to keep count;
        }
    };

    println!("Final result of counter is: {result}");

    counter = 0;

    // Essentially a double for loop
    // Really like how you can break for an outer loop within an inner loop

    'counting_up: loop { // give outer loop name 'counting_up;

        println!("Count = {counter}"); // print counter on ever entry
        let mut remaining = 10; // reset remaining on every entry

        loop {
            // Will only print remaining on 10 and 9
            println!("Remaining is {remaining}");
            if remaining == 9 {
                break;
            }

            if counter == 2 {// once counter == 2 we break out of all loops
                break 'counting_up;
            }
            remaining -= 1;
        }
        counter += 1;
    }
    println!("End count is: {counter}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("Broken out of while loop");

    let a = [2; 10];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }

    // kinda like a for each loop,
    // for val within range of 1 and 5 inclusive
    // .rev() method reversed it from 5 -> 1
    for number in (1..=5) {
        println!("{number}");
    }
    println!("Broken out of for loop");
}