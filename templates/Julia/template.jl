#!/usr/bin/env -C julia -t auto

"""
This module showcases a number of powerful Julia idioms, including:
    - using the shebang to allow julia to use the available threads rather than 
      running in its default single-threaded module
    - using a statically typed struct to structure data and catch errors instead
      of using a Tuple or a Dict, which can be type Any.
    - using tuple unpacking to capture multiple values return by a function
    - using `_` to throw away one of those values
    - using `precompile()` to (you guessed it) precompile the function rather than
      running it with the default Julia just-in-time compiler. This can help get
      around the so-called "time-to-first-plot" problem. The first time you run a
      function in a Julia script, it has to compile and run that function. By
      using a forced ahead-of-time compilation, and thus decoupling compilation from
      the first runtime, you can get better performance.
"""

# Define a UserData struct to hold user information
struct UserData
    name::String
    age::UInt8
end

"""
    validate_age(user::UserData)

Validate the age of a user.

# Arguments
- `user::UserData`: A UserData instance whose age needs to be validated.

# Returns
- `Tuple{Union{Nothing, UserData}, Union{Nothing, String}}`: 
  Returns `(user, nothing)` if age is valid, else returns `(nothing, "error message")`.

"""
function validate_age(user::UserData)
    if user.age < 18
        return nothing, "User $(user.name) is under 18."
    end
    if user.age > 120
        return nothing, "User $(user.name)'s age $(user.age) is not valid."
    end
    
    return user, "throw away demo", nothing

end
precompile(validate_age, (UserData,))

function main()
    # Create UserData instances
    user1 = UserData("Alice", 25)
    user2 = UserData("Bob", 15)
    user3 = UserData("Charlie", 130)

    # Create an array of users
    users = [user1, user2, user3]

    # Validate each user
    for user in users
        valid_user, _, err = validate_age(user)
        if err === nothing
            println("User $(valid_user.name) has a valid age.")
        else
            println(err)
        end
    end
end
precompile(main)

# Run the main function
main()
