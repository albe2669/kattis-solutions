package main

import (
	"fmt"
)

func main() {
	var n int
	fmt.Scanln(&n)

	var total int = 0

	for i := 0; i < n; i++ {
		var length int
		fmt.Scanln(&length)

		total += length - 1
	}

	fmt.Println(total + 1)
}
