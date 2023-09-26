#!/usr/bin/env python3

"""
This module showcases a number of powerful python idioms, including:
    - exactly zero global variables
    - docstrings for every function and class.
    - a simple dataclass to store and validate information without the potential pitfalls of tuples
      and dicts (namely, that they can by any type, so a linter can't prevent you from misusing
      functions on them).
    - pervasive type hinting, which allows linters like Pylance to catch errors that you might not 
      otherwise have detected until runtime (or worse, until some other time in the future!)
    - the aforementioned Result type, which can be used for elegant, Rust-style error handling
    - command line argument construction and parsing with `argparse`
    - structural pattern matching via the `match`-`case` block, which was introduced in Python 3.10,
      which we use to handle errors.
    - no unnecessary usage of `else` statements when the `if` statement calls `return`.
    - using `if` statements to test the "unhappy" condition first. This helps reduce nesting, which
      can be particularly pernicious in Python because of its use of whitespace to denote 
      an unfinished statement.
    - handling errors as one of the main function's most important responsibilities.
    - using concise f-strings to embed strings within strings via their variable bindings.
    - the `if __name__ == "__main__":` block at the bottom, which signals to collaborators that this
      is a script and not a library.

Example:
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

    # parse command line arguments
    args = parse_args()

    # use structural pattern matching to create a new `UserData`
    # instance or handle any errors
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
        if result.is_ok():
            print(f"User {result.unwrap().name} has a valid age.")
        else:
            print(result.unwrap_err())

if __name__ == "__main__":
    main()
