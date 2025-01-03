// Example Struct for a user account
struct User {           //inside the curly brackets we call it fields
    active: bool,       //fields are the different names and types of
    username: String,   //data
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User{
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


fn main() {
    // Creating a User Instance by defining every instance of the Struct
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // With a dot Notation we can get a Specify Value from the user
    let is_user_one_active = user1.active;

    // When the Instance is mutable we can also change Values in the fields with a dot Notation
    let mut user2 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user2.active = false;
    // When we want to change Values of the Instance the whole Instance needs to be Mutable. Because
    // you can't just have one field mutable

    // We can also Build a function, which creates automatically user Instances

    build_user("test@test.de".to_string(),"Kevin".to_string());
    print!("{}",build_user("test@test.de".to_string(),"Kevin".to_string()).username);

    // Creating Instances from Other Instances with Struct Update Syntax
    let user3 = User{
        email: String::from("another@badmail.com"),
        ..user1
    };

    //Using Tuple Structs Without Named Fields to Create Different Types

    fn main() {
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
    }

    //Unit-Like Structs Without Any Fields
}


