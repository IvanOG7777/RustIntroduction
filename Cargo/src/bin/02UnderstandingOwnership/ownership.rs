fn main() {
    {
        let s = "hello"; // valid within this scope

        println!("{s} from inner scope");
    } // variable s is dropped here once inner scope ends

    // println!("{s} from outer scope"); // invalid unless s is redeclared in outer scope
}