struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Paint(i32, i32, i32);

fn main() {
    let user1 = User {
        active: true,
        username: String::from("Kim"),
        email: String::from("email@somewhere.to"),
        sign_in_count: 2,
    };

    let user2 = User {
        email: String::from("new@something.where"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Color(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

