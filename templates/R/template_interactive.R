
# A NOTE ON HOW R IS TYPICALLY USED
# -------------------------------- #
# Most work in R happens interactively in RStudio or a notebook, where a user
# runs the script line by line to perform exploratory data analysis, statistics,
# or data visualization. In this context, it is unusual to define and compile
# functions as we did in the script template. Instead of parsing arguments from
# the command line, input file paths are often hardcoded. Most if not all
# variables tend to be globals in these scripts.

# Other scripting languages like Python and Julia strongly recommend against
# writing scripts with any of these characteristics. Indeed, interactive R
# scripts can be cumbersome, difficult to read, disorganized, and very slow. And
# that's okay! Most writers of R are not software engineers; they are scientists
# looking to write code that quickly provides some kind of insight. They want to
# think about the data, not the program itself. R shines in this context. In
# recognition of how R is typically used, we thus include a script version
# alongside this interactive version of the same code. The script version show
# cases some ways to improve the performance and organization of R scripts,
# whereas the interactive version will be more in line with what the typical R
# user expects a script to look like.
# -------------------------------- #

# Create some user data
user1 <- list("Alice", 25)
user2 <- list("Bob", 15)
user3 <- list("Charlie", 130)

# Store users in a list
users <- list(user1, user2, user3)

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

# Validate each user
for (user in users) {
  result <- validate_age(user)
  cat(result, "\n")
}
