package main

import "fmt"

func slice_print(list []int) {
	for _, i := range list {
		fmt.Printf("%2d ", i)
	}
	fmt.Printf("\n")
}

func main() {
	var list []int

	list = append(list, 1, 1, 2, 2)
	slice_print(list)
	slice_print(list[:2])
	slice_print(list[2:])
}
