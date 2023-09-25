#!/usr/bin/env -C julia -t auto

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
    elseif user.age > 120
        return nothing, "User $(user.name)'s age $(user.age) is not valid."
    else
        return user, nothing
    end
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
        valid_user, err = validate_age(user)
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
