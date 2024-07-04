#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

// Struct without named fields
#[derive(Debug)]
struct points(i32, i32, i32);

// Associated Function
impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }

    fn deactivate(&mut self) {
        self.active = false;
    }
}

pub fn use_struct() {
    let p1 = Person {
        first_name: String::from("John"),
        last_name: "Doe".to_string(),
        age: 30,
    };

    println!("{:?}", p1);
    println!("First name: {}", p1.first_name);
    println!("Last name: {}", p1.last_name);
    println!("Age: {}", p1.age);

    let mut new_user = User::new(
        String::from("rahul_trivedi"),
        String::from("rahul@example.com"),
        String::from("https://example.com/rahul_trivedi"),
    );
    println!("{:?}", new_user);
    println!(
        "Username: {}'s status is {}",
        new_user.username, new_user.active
    );
    new_user.deactivate();
    println!(
        "Username: {}'s status is {}",
        new_user.username, new_user.active
    );

    let username = String::from("john_doe");
    let email = String::from("john@example.com");
    let uri = format!("https://example.com/{}", username);
    let active = true;

    // keep variable name same as struct to use directly like a constructor
    let john_doe = User {
        username,
        email,
        uri,
        active,
    };

    println!("{:?}", john_doe);

    let my_point = points(10, 20, 30);
    println!("Points: {}", my_point.1);
}
