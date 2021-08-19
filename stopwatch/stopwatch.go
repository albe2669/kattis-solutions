package main

import (
	"fmt"
)

func main() {
	var presses int

	fmt.Scanln(&presses)

	if presses % 2 != 0 {
		fmt.Println("Still running")
		return
	}

	var total int64 = 0
	var startTime int64 = 0

	for i := 0; i < presses; i++ {
		var num int64 = 0
		fmt.Scanln(&num)

		if i % 2 == 0 {
			startTime = num
		} else {
			total += num - startTime
		}
	}

	fmt.Println(total)
}
