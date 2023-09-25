/// This module contains a simple example that demonstrates how to use the `Result` type in Rust.
/// It defines a `UserData` struct and a `validate_age` function that checks the validity of the `age` field.
mod user {
    #[derive(Debug)]
    pub struct UserData {
        pub name: String,
        pub age: u8,
    }

    /// Validates the age of a user.
    ///
    /// # Arguments
    ///
    /// * `user` - A UserData instance whose age needs to be validated.
    ///
    /// # Returns
    ///
    /// * `Result<UserData, String>` - Returns `Ok(UserData)` if age is valid, else returns `Err(String)`.
    pub fn validate_age(user: &UserData) -> Result<&UserData, String> {
        if user.age < 18 {
            Err(format!("User {} is under 18.", user.name))
        } else if user.age > 120 {
            Err(format!("User {}'s age {} is not valid.", user.name, user.age))
        } else {
            Ok(user)
        }
    }
}

fn main() {
    use user::{UserData, validate_age};

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
