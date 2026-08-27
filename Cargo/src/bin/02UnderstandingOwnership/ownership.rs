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

    let new_s1 = String::from("Hello");
    let new_s2 = new_s1.clone();

    
}