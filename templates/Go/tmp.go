package main

import (
	"errors"
	"fmt"
)

// UserData struct to hold user information
type UserData struct {
	name string
	age  uint8
}

// ValidateAge function to validate user age
func ValidateAge(user *UserData) (*UserData, error) {
	if user.age < 18 {
		return nil, errors.New(fmt.Sprintf("User %s is under 18.", user.name))
	} else if user.age > 120 {
		return nil, errors.New(fmt.Sprintf("User %s's age %d is not valid.", user.name, user.age))
	} else {
		return user, nil
	}
}

func main() {
	// Create UserData instances
	user1 := &UserData{name: "Alice", age: 25}
	user2 := &UserData{name: "Bob", age: 15}
	user3 := &UserData{name: "Charlie", age: 130}

	// Create a slice of users
	users := []*UserData{user1, user2, user3}

	// Validate each user
	for _, user := range users {
		validUser, err := ValidateAge(user)
		if err != nil {
			fmt.Println(err)
		} else {
			fmt.Printf("User %s has a valid age.\n", validUser.name)
		}
	}
}
