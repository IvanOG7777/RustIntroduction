fn main() {
    {
        let s1 = String::from("hello");

        let length = calculate_length(&s1);

        println!("The length of {s1} is: {length}");
    }

    {
        let s = String::from("Hello");

        let mut s2 = String::from("Hello"); // declaring that s2 itself can be mutable

        // change(&s); // change takes reference of stack s pointer, cant change though

        change_mut(& mut s2); // pass in a mutable reference to s2 stack pointer

        println!("{s}");
        println!("{s2}");
    }

    {
        let mut s = String::from("hello");

        let r1 = &mut s;
        // let r2 = &mut s; // cannot do a double borrow on a variable

        println!("{r1}");
    }

    let mut s = String::from("hello");
    {
        let r1 = &mut s;

        println!("r1 from inner scope has mutable reference of s: {r1}");
        println!("r1 has gone out of scope and isn't holding mutable borrow anymore");
    }

    let r2 = &mut s;
    println!("r2 from outer scope has mutable reference of s: {r2}");

    {
        // immutable vs mutable references;
        // you can have N amount of immutable references to a variable on the stack as reader
        // you cannot have both immutable and mutable references of a single variable unless either mutable or immutable variable has been dropped or out of scope

        let mut s = String::from("hello");

        let r1 = &s;
        let r2 = &s;
        // let r3 = &mut s; // error

        // println!("r1: {r1}, r2: {r2}, r3: {r3}"); // invlid print

        println!("r1: {r1}, r2: {r2}");

        let r3 = &mut s; // this is valid since above immutable references have been dropped
        println!("r3: {r3}");
    }

    // dangling reference
    {
        // let ref_to_nothing = dangle(); ref_to_nothing receives address to dropped variable from within function(invalid)

        let mut ref_to_something = no_dangle();
    }
}

// function only takes reference of the address where s1 is stored (borrow)
// function can then access metadata and heap allocated data that s1 hold
fn calculate_length(string: &String) -> usize {
    let length = string.len();

    length
}

// fn change(string: &String) {
//     string.push_str(", World"); //cant change because passed string is only a borrow. Essentially read only
// }

// function doesnt need ownership
fn change_mut(passed_string: &mut String) { // only asks for a mutable borrow of a String object
    passed_string.push_str(", World");
}

// invalid since we are trying to return a reference to s within the function call
// when function call returns the itself and the stack allocated are dropped
// the function is returning an address that no longer exits
// fn dangle() -> &String {
//     let s= String:: from("hello");
//
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}