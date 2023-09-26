#!/usr/bin/env Rscript

options(warn = 1)
suppressPackageStartupMessages({

  require(compiler)
  require(argparse)

})

# Define a function to create a user "object" as a list
create_user <- compiler::cmpfun(function(name, age) {
  list(name = name, age = age)
})

# Function to parse command line arguments
parse_args <- compiler::cmpfun(function() {
  parser <- ArgumentParser(description = "An example script for argparse in R")
  parser$add_argument("--name", required = TRUE, default = "John",
                      help = "Name of the new user")
  parser$add_argument("--age", type = "integer", required = TRUE, default = 35,
                      help = "Age of the new user")
  args <- parser$parse_args()

  return(args)
})

# Function to validate the age of a user
validate_age <- compiler::cmpfun(function(user) {
  if (user$age < 18) {
    return(paste("User", user$name, "is under 18."))
  }
  if (user$age > 120) {
    return(paste("User", user$name, "age", user$age, "is not valid."))
  }
  return(paste("User", user$name, "has a valid age."))
})

# Main function to run the code
main <- compiler::cmpfun(function() {

  # parse command line information
  args <- parse_args()

  # assertion to check that args are not blank, NA, or partially missing
  stopifnot(!(TRUE %in% is.na(args)))
  stopifnot(length(args) == 2)

  # Create some user data
  user1 <- create_user("Alice", 25)
  user2 <- create_user("Bob", 15)
  user3 <- create_user("Charlie", 130)
  user_from_args <- create_user(args[1], args[2])

  # Store users in a list
  users <- list(user1, user2, user3, user_from_args)

  # Validate each user
  for (user in users) {
    result <- validate_age(user)
    cat(result, "\n")
  }
})

# Run the main function
main()
