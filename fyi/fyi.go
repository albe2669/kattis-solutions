package main

import (
	"fmt"
)

func main() {
	var in string
	fmt.Scanln(&in)

	for i := 0; i < 3; i++ {
		if string(in[i]) != "5" {
			fmt.Println("0")
			return
		}
	}
	
	fmt.Println("1")
}
