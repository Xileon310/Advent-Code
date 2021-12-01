package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func main() {

	// Take the file
	f, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}

	scanner := bufio.NewScanner(f)

	// Read first number
	scanner.Scan()
	old_value, err := strconv.Atoi(scanner.Text())

	increase := 0

	// Read line by line
	for scanner.Scan() {
		new_value, _ := strconv.Atoi(scanner.Text())

		if old_value < new_value {
			increase++
		}

		old_value = new_value
	}

	fmt.Println(increase)

	defer f.Close()
}
