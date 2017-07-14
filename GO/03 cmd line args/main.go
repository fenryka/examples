package main

import (
	"flag"
	"fmt"
)

func main() {
	fmt.Println("Hello world")

	numbPtr := flag.Int("numb", 42, "an int")
	strPtr  := flag.String("str", "Pants", "an int")

	flag.Parse()

	fmt.Println("arg 1: ", *numbPtr)
	fmt.Println("arg 2: ", *strPtr)
}
