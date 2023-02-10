package main

import (
	"fmt"
)

func main() {
	var wc, hc, ws, hs int
	fmt.Scanln(&wc, &hc, &ws, &hs)

	if ws + 2 <= wc && hs + 2 <= hc {
		fmt.Println("1")
	} else {
		fmt.Println("0")
	}
}
