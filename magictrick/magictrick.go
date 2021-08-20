package main

import (
	"fmt"
)

func main() {
	var in string
	fmt.Scanln(&in)

	duplicates := false
	m := map[byte]bool{}
	
	for i := 0; i < len(in); i++ {
		if !m[in[i]] {
			m[in[i]] = true
		} else {
			duplicates = true
			break
		}
	}

	if duplicates {
		fmt.Println("0")
	} else {
		fmt.Println("1")
	}
}
