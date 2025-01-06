#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someome@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    let email = String::from("another@example.com");
    let username = String::from("anothername");
    let user2 = build_user(email, username);
    
    let user3 = User {
        email: String::from("email3@example.com"),
        username: user1.username,
        ..user1
    };

    println!("user2 = {:?}",user2);
    println!("user3 = {:?}",user3);
    println!("user1.active = {}", user1.active);
    println!("user1.sign_in_count = {}", user1.sign_in_count);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
