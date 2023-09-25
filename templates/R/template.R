#!/usr/bin/env Rscript

# Define a function to create a user "object" as a list
create_user <- compiler::cmpfun(function(name, age) {
  list(name = name, age = age)
})

# Function to validate the age of a user
validate_age <- compiler::cmpfun(function(user) {
  if (user$age < 18) {
    return(paste("User", user$name, "is under 18."))
  } else if (user$age > 120) {
    return(paste("User", user$name, "age", user$age, "is not valid."))
  } else {
    return(paste("User", user$name, "has a valid age."))
  }
})

# Main function to run the code
main <- compiler::cmpfun(function() {
  # Create some user data
  user1 <- create_user("Alice", 25)
  user2 <- create_user("Bob", 15)
  user3 <- create_user("Charlie", 130)

  # Store users in a list
  users <- list(user1, user2, user3)

  # Validate each user
  for (user in users) {
    result <- validate_age(user)
    cat(result, "\n")
  }
})

# Run the main function
main()
