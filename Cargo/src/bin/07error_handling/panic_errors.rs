pub mod recoverable_errors;

fn main() {

    // panic!("Crash and burn"); // causes immediate abortion of program, no back tracking the stack due to [profile.release] panic = 'abort'

    {
       let v =vec![1, 2, 3];

        v[99];
    }

}