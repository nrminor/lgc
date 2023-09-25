fn main() {
    use lib::user::{UserData, validate_age};

    // Create UserData instances
    let user1 = UserData { name: String::from("Alice"), age: 25 };
    let user2 = UserData { name: String::from("Bob"), age: 15 };
    let user3 = UserData { name: String::from("Charlie"), age: 130 };

    // Validate each user
    for user in vec![&user1, &user2, &user3] {
        match validate_age(user) {
            Ok(valid_user) => println!("User {} has a valid age.", valid_user.name),
            Err(err) => println!("{}", err),
        }
    }
}
