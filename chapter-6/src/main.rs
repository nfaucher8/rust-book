struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Unit Struct
struct AlwaysEqual;

fn main() {
    let mut user1 = build_user(String::from("someusername123"),String::from("someone@example.com"));

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("user2email@example.com"),
        ..user1
    };

    println!("{}", user1.email);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}