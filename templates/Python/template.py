#!/usr/bin/env python3

"""
This module showcases a number of powerful python idioms alongside some newer Python features.
Read more in the provided template readme.

Example Invocation:
    $ python3 template.py Brian 22
"""

import sys
import argparse
from dataclasses import dataclass
from result import Result, Ok, Err

@dataclass
class UserData:
    """A data class for holding user information."""
    name: str
    age: int

def parse_args() -> Result[argparse.Namespace, str]:
    """
    Parses command line arguments.

    Returns:
        Result[argparse.Namespace, str]: Returns Ok(argparse.Namespace) if args could 
        be parsed, else returns Err(str).
    """
    parser = argparse.ArgumentParser(description="Add a new user with name and age.")
    parser.add_argument("--name", type=str, required=True, help="Name of the user")
    parser.add_argument("--age", type=int, required=True, help="Age of the user")
    args = parser.parse_args()
    if len(sys.argv) < 2:
        return Err("Command line arguments failed to parse.")
    return Ok(args)

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

def main() -> None:
    """
    Main takes the result from validate_age() and exposes any errors.
    """

    # the idioms in this script require Python 3.10 or later
    if sys.version_info <= (3, 10):
        sys.exit("Script requires Python 3.10 or higher")

    # parse command line arguments with `argparse`
    args = parse_args()

    # use structural pattern matching to create a new `UserData`
    # instance or handle any errors. This guarantees that we won't
    # accidentally pass an empty or corrupted UserData instance to
    # the function `validate_age()`, which would produce unexpected
    # errors in that case
    match args:
        case Ok(args):
            user_from_args = UserData(args.unwrap().name,  args.unwrap().age)
        case Err(args):
            sys.exit(f"{args.unwrap_err()}")
        case _:
            sys.exit("Demoing how to handle all other cases.")

    # hardcode in a few more users
    user1 = UserData("Alice", 25)
    user2 = UserData("Bob", 15)
    user3 = UserData("Charlie", 130)

    # loop through each user and make sure all their ages make sense
    for user in [user1, user2, user3, user_from_args]:
        result = validate_age(user)
        if not result.is_ok():
            print(result.unwrap_err())
            continue
        print(f"User {result.unwrap().name} has a valid age.")

if __name__ == "__main__":
    main()
