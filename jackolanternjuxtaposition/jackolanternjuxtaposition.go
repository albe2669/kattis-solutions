package main

import (
	"fmt"
)

func main() {
	var eyes, nose, mouth int

	fmt.Scanln(&eyes, &nose, &mouth)
	fmt.Println(eyes * nose * mouth)
}
