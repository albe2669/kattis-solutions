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

	var n int
	scanf("%d\n", &n)

	nums := make([]int, n)
	for i := 0; i < n; i++ {
		scanf("%d\n", &nums[i])
	}

	for i := n-1; i >= 0; i-- {
		printf("%d\n", nums[i])
	}
}
