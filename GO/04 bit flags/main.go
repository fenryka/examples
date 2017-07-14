package main

import "fmt"

func main() {
	var a byte
	a = 0
	// turn on 2nd bit, OR
	a |= 2
	fmt.Printf("%b\n", a)
	// then the 4th
	a |= 8
	fmt.Printf("%b\n", a)
	// then turn off the 2nd
	a ^= 2
	fmt.Printf("%b\n", a)
	fmt.Printf("reset to 0\n")
	a = 0
	a |= 32 | 8 | 2
	fmt.Printf("%b\n", a)
}
