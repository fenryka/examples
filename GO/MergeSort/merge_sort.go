package main

import "fmt"

func print_list(hdr string, list []int) {
	fmt.Printf("%s", hdr)
	for _, val := range list {
		fmt.Printf("%2d ", val)
	}
	fmt.Printf("\n")
}

func merge_sort(list []int) []int {
	if len(list) == 1 {
		return list
	}

	pivot := len(list) / 2

	l := merge_sort(list[pivot:])
	r := merge_sort(list[:pivot])

	var tlist []int
	lidx := 0
	ridx := 0

	for {
		if l[lidx] < r[ridx] {
			tlist = append(tlist, l[lidx])
			lidx += 1
			if lidx == len(l) {
				tlist = append(tlist, r[ridx:]...)
				break
			}
		} else {
			tlist = append(tlist, r[ridx])
			ridx += 1
			if ridx == len(r) {
				tlist = append(tlist, l[lidx:]...)
				break
			}
		}
	}
	return tlist
}

func main() {
	var list []int

	list = append(list, 4, 3, 5, 5, 89,3, -1, 16, 1, 2, 1, 8)

	print_list("pre sort:  ", list)
	list = merge_sort(list)
	print_list("post sort: ", list)
}
