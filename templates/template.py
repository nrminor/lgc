#!/usr/bin/env python3

"""
This module contains a simple example of using the Result type from the `result` package in Python.
It defines a UserData data class and a validate_age function that checks the validity of the 
age field.

Example:
    $ python script_name.py
"""

from dataclasses import dataclass
from result import Result, Ok, Err

@dataclass
class UserData:
    """A data class for holding user information."""
    name: str
    age: int

def validate_age(user: UserData) -> Result[UserData, str]:
    """
    Validates the age of a user.

    Args:
        user (UserData): The user whose age needs to be validated.
    
    Returns:
        Result[UserData, str]: Returns Ok(UserData) if age is valid, else returns Err(str).
    """
    if user.age < 18:
        return Err(f"User {user.name} is under 18.")

    if user.age > 120:
        return Err(f"User {user.name}'s age {user.age} is not valid.")

    return Ok(user)

def main():
    """
    Main takes the result from validate_age() and exposes any errors.
    """
    user1 = UserData("Alice", 25)
    user2 = UserData("Bob", 15)
    user3 = UserData("Charlie", 130)

    for user in [user1, user2, user3]:
        result = validate_age(user)
        if result.is_ok():
            print(f"User {result.unwrap().name} has a valid age.")
        else:
            print(result.unwrap_err())

if __name__ == "__main__":
    main()
