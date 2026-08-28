fn main() {
    {
        let s = "hello"; // valid within this scope

        println!("{s} from inner scope");

        let sh = String::from("Hello");
        println!("{sh} from inner scope again");
    } // variable s is dropped here once inner scope ends

    // println!("{s} from outer scope"); // invalid unless s is redeclared in outer scope

    // String object which holds pointer to heap where "Hello" is stored
    let mut s = String::from("Hello");
    s.push_str(", World!"); // method to push new string literal onto current string, rust will allocate more memory if current pointer doesnt have enough space for addition
    println!("{s}");

    let x = 5; // x owns 5 on stack
    let y = x; // y owns a copy of 5

    let s1 = String::from("Hello"); // s1 String object owns "Hello" on the heap
    let s2 = s1; // s1 then transfers ownership to s2. s1 drops its ownership

    // println!("Using s1: {s1}"); error s1 ownership has been moved
    println!("Using s2: {s2}");

    let mut s = String::from("hello");
    s = String::from("Ahoy");

    println!("{s}");


    // Allocates new metadata pointer on the stack for new_s2;
    // Allocates same length, capacity and a new address for where copied "Hello" will live
    let new_s1 = String::from("Hello");
    let new_s2 = new_s1.clone();// does a deep copy of new_s1 on the heap;

    println!("This is a deep copy");
    println!("Value of new_s1 is {new_s1}. Value of new_s2 is {new_s2}");

    // Ownership and Functions
    {
        let s = String::from("hello"); // s cones into scope;
        takes_ownership(s); // s's ownership over the metadata at current pointer are given to takes_ownership();
        // s goes out of scope.

        // println!("Value at s is: {s}"); invalid call since s doesn't own metadata anymore since takes_ownership has ownership over s;

        let x = 5; // x comes into scope;
        makes_copy(x); //i32 values have a Copy trait, x does not move out of the scope
        // makes_copy only takes a copy of what held at x. x is still valid after function call

        println!("Value at x is: {x}"); // Valid call
    }

    //Return values and Scope
    {
        let s1 = gives_ownership(); // function call gives it own ownership from within to s1

        let s2 = String::from("hello");

        let s3 = takes_and_gives_back(s2); // takes_and_gives_back() takes ownership from s2 and gives it to s3;

        println!("Value of s1 is: {s1}");

        // println!("Value of s1 is: {s2}"); invalid call since takes_and_gives_back() takes s2 ownership over metadata at s2 and gives it to s3;

        println!("Value of s1 is: {s3}");

    }
}

fn takes_ownership(string: String) { // function comes into scope
    println!("{string}"); // once it's done passed value and itself are dropped out of scope.
}

fn makes_copy(integer: i32) { // function comes into scope
    println!("{integer}"); // Since function has its own copy of passed value
} // only function and copy of passed value are dropped from scope

fn gives_ownership() -> String {
    let string = String::from("yours"); // creates its own heap allocated string

    string // returns and gives ownership to outer function call
}

fn takes_and_gives_back(string: String) -> String { // takes ownership if passed heap allocated string
    string // returns it to wherever function was called
}