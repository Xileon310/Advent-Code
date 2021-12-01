package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

/*
	Example:

	1	A
	2	A	B
	3	A	B	C
	4		B	C	D
	5			C	D	E
	6				D	E	F
	7					E	F	G

	Description:

	Program will read first 3 numbers and will do the necessary operations. Next,
	it will start a for loop until file end where it will be reading each value
	line by line and adding it to the two previous values. Also, it will check
	a condition of the last positions on the slice that have completed the three
	sums, if they satisfy the condition then it will increase the counter.

*/
func main() {

	// Take the file
	f, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}

	scanner := bufio.NewScanner(f)

	// Using slices to save the three-measurement
	values := []int{}
	var v int

	// Read first number
	scanner.Scan()
	v, err = strconv.Atoi(scanner.Text())
	values = append(values, v)

	// Read second number
	scanner.Scan()
	v, err = strconv.Atoi(scanner.Text())
	values[0] += v
	values = append(values, v)

	// Read third number
	scanner.Scan()
	v, err = strconv.Atoi(scanner.Text())
	values[0] += v
	values[1] += v
	values = append(values, v)

	// Initialize increase var and index of slice
	increase := 0
	index := len(values)

	// Read line by line
	for scanner.Scan() {
		// Get value and add to the two previous values
		v, _ = strconv.Atoi(scanner.Text())
		values[index-2] += v
		values[index-1] += v

		// Compare between the values that have completed their sums
		if values[index-3] < values[index-2] {
			increase++
		}

		values = append(values, v)
		index++
	}

	fmt.Println(increase)

	defer f.Close()
}
