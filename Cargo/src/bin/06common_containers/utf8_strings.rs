use std::fmt::format;

fn main() {
    let data = "initial contents";

    // creates a string literal to String object
    let string = data.to_string();
    let string = "initial contents".to_lowercase();

    //similar to this:
    let string = String::from("initial contents");

    // since String object is UTF-8 encoded we can do stuff like this
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");
    s += "foo";

    println!("{s}");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l'); // push method only takes single character
    println!("{s}");

    let s1 = String::from("Hello");
    let s2 = String::from("world");
    let s3 = s1 + &s2;

    // println!("{s1}"); //  error since s3 takes ownership
    println!("{s2}"); // valid since s3 only takes a borrow after s3 is out of scope handle back borrow
    println!("{s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{s}");

   // println!("{s1}"); // error again since s takes ownership of s1
    // valid since s only takes a borrow of s2 and s3 and gives ownership back after
    println!("{s2}");
    println!("{s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}"); // format macro, no need for redundant s1 + "-" + &s2 + "-" + &s3;

    println!("{s}");

    println!("{s1}");
    println!("{s2}");
    println!("{s3}");

    let s1 = String::from("Hello");
    // let h = s1[0]; // since String is utf-8 a character within the string can go past an index thus not allowed
    let h = &s1[0..2];
    println!("{h}");
}