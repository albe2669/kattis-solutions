package main

import (
	"fmt"
)

func main() {
	var n, p, s int
	fmt.Scanln(&n, &p, &s)

	for i := 0; i < s; i++ {
		var m int
		fmt.Scan(&m)
		hasPred := false

		for j := 0; j < m; j++ {
			var num int
			fmt.Scan(&num)

			if num == p {
				hasPred = true
			}
		}

		if hasPred {
			fmt.Println("KEEP")
		} else {
			fmt.Println("REMOVE")
		}
	}
}
