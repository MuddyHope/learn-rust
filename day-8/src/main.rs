// Initialization of struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Avatar {
    active: bool,
    username: String,
    email: String,
    health: u64,
}


// Implementation
impl Avatar {
    fn new(username: String, email: String) ->  Avatar{
        Avatar {
            active: true,
            username, 
            email,
            health: 100
        }
    }
}

fn main() {
    // Instantiation of user struct
    let user1 = User{
        active: true,
        username: String::from("my_user"),
        email: String::from("my_user@gmail.com"),
        sign_in_count: 1
    };

    println!("Hello, world! {}", user1.username);

    // Making the object of the struct mutable
    let mut user2 = User{
        active: true,
        username: String::from("my_user_2"),
        email: String::from("my_user@_2gmail.com"),
        sign_in_count: 1
    };

    user2.username = String::from("kjalsdjf");
    println!("Hello World, {}", user2.username);


    let avatar1 = Avatar::new(String::from("anotheruser"), String::from("anotheruser@gmail.com"));

    println!("Hello Avatar from impl {}", avatar1.username);

}
