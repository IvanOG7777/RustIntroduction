mod different_instances;

// similar declaration to typescript interfaces
struct User {
    active: bool,
    user_name: String,
    email: String,
    sign_in_count: u64,
}
fn main() {

    // created an instance of User object to variable name user1
    // immutable User
    let user1 = User {
        active: true,
        user_name: String::from("Ivan Argueta"),
        email: String::from("IvanOG@gmail.com"),
        sign_in_count: 1,
    };

    // Mutable User
    // entire user must be mutable in order to change a specific variable within
    //CANT declare a single variable as mutable within struct rust doesn't allow i
    let mut user2 = User {
        active: true,
        user_name: String::from("Ivan Argueta"),
        email: String::from("IvanOG@gmail.com"),
        sign_in_count: 1,
    };

    user2.email = String::from("AnotherEmail@gmail.com");

    let user3 = build_user(String::from("email.com"), String::from("Ivan"));

    let user4 = build_user_short_hand(String::from("newEmail.com"), String:: from("Ivan"));

    // Copy values from other declared structs to new declareation
    let user5 = User {
        active: user1.active,
        user_name: user2.user_name,
        email: user3.email,
        sign_in_count: user4.sign_in_count,
    };

    // create new user, with new email
    // copy other elements from user1 to user6
    let user6 = User {
        email: String::from("user6@email.com"),
        ..user1
    };
}

fn build_user(passed_email: String, passed_user_name: String) -> User {
    User {
        active: true,
        user_name: passed_user_name,
        email: passed_email,
        sign_in_count: 1
    }
}

// In order to shorthand a struct must pass variable names must match struct names
fn build_user_short_hand(email: String, user_name: String) -> User {
    User {
        active: true,
        user_name,
        email,
        sign_in_count: 1,
    }
}