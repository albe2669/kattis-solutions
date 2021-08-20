package main

import (
	"bufio"
	"os"
	"fmt"
)

var reader *bufio.Reader = bufio.NewReader(os.Stdin)
var writer *bufio.Writer = bufio.NewWriter(os.Stdout)
func printf(f string, a ...interface{}) { fmt.Fprintf(writer, f, a...) }
func scanf(f string, a ...interface{}) { fmt.Fscanf(reader, f, a...) }

func main() {
	defer writer.Flush()

	var n, result int = 0,0
	scanf("%d\n", &n)

	var prev string
	scanf("%s\n", &prev)

	for i := 1; i < n; i++ {
		var answer string
		scanf("%s\n", &answer)

		if prev == answer {
			result += 1
		}

		prev = answer
	}

	printf("%d\n", result)
}
