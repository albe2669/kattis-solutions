package main

import (
	"bufio"
	"fmt"
	"os"
)

var reader *bufio.Reader = bufio.NewReader(os.Stdin)
var writer *bufio.Writer = bufio.NewWriter(os.Stdout)

func printf(f string, a ...interface{}) { fmt.Fprintf(writer, f, a...) }
func scanf(f string, a ...interface{})  { fmt.Fscanf(reader, f, a...) }

func swap(a bool, b bool) (bool, bool) {
	return b, a
}

func main() {
	// STDOUT MUST BE FLUSHED MANUALLY!!!
	defer writer.Flush()

	var left, middle, right bool
	left = true

	var moves string
	scanf("%s\n", &moves)

	for _, move := range moves {
		if move == 'A' {
			left, middle = swap(left, middle)
		} else if move == 'B' {
			middle, right = swap(middle, right)
		} else {
			left, right = swap(left, right)
		}
	}

	if left {
		printf("1\n")
	} else if middle {
		printf("2\n")
	} else {
		printf("3\n")
	}
}
