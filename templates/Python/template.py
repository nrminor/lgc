#!/usr/bin/env python3

"""
This module showcases a number of powerful python idioms alongside some newer Python features:

    ### General Good Practices in Python
    - exactly zero global variables (if there were any, we'd make sure they are never modified and
      denote them as constants by putting their variable names in all caps). Like many of the idioms
      below, Python itself does not natively support constants--but many Python linters do!
    - docstrings for every function and class.
    - command line argument construction and parsing with `argparse`
    - no unnecessary usage of `else` statements when the `if` statement calls `return`.
    - main() takes and returns no arguments. It is simply the entrypoint of the program.
    - using concise f-strings to embed strings within strings via their variable bindings.
    - using `if` statements to test the "unhappy" condition first. This helps reduce nesting, which
      can be particularly pernicious in Python because of its use of whitespace to denote 
      an unfinished statement.
    - the `if __name__ == "__main__":` block at the bottom, which signals to collaborators that this
      is a script and not a library.

    ### Making the most of linters with type annotations and other "Rust-like" conventions
    - a simple dataclass to store and validate information without the potential pitfalls of tuples
      and dicts (namely, that they can by any type, so a linter can't prevent you from misusing
      functions on them).
    - pervasive type hinting, which allows linters like Pylance to catch errors that you might not 
      otherwise have detected until runtime (or worse, until some other time in the future!)
    - the Result type, which can be used for elegant, Rust-style error handling. You could also use
      `Union{T, str}` or `T | str` to do the same thing. Also keep in mind the `Optional[T]` from
      the `typing` library, which is the same as using `Union{T, None}` or `T | None` as the return
      type.
    - structural pattern matching via the `match`-`case` block, which was introduced in Python 3.10,
      to handle errors gracefully. This is once again an influence from Rust.
    - handling errors as one of the main function's most important responsibilities.

    ### Other recommendations
    - In many cases, Python is used for data exploration and analysis, often with data frames. Data
      frames are often parsed from CSV or Excel files and subsequently queried via column names or
      column indices. In both cases, it is wise to use `assert` statements, with a helpful message,
      to double-check that the expected column name or index is actually present. This makes runtime
      errors a little more helpful.
    - When performance matters with big data frames, use Polars instead of Pandas.
    - When working with data, creating too many visualizations, similar to assertions, is preferable
      to creating too few. We recommend the Seaborn package for doing so. 
    - Parsing datetimes can be a nightmare in any language. We recommend the pendulum package for
      doing so.
    - Python is an excellent glue language. Rather than performing all tasks in Python, which may be
      slower as a result, we recommend making liberal use of subprocesses to run other purpose-build
      command line tools in your Python scripts. Just make sure to avoid opening up a security risk
      by using the option `shell = True`.

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
