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

	var in string
	scanf("%s\n", &in)

	suits := map[string]int{
		"P": 13,
		"K": 13,
		"H": 13,
		"T": 13,
	}
	seen := make(map[string]bool)
	i := 0

	for i < len(in) {
		var suit, card string
		suit = string(in[i])
		card = suit + string(in[i+1]) + string(in[i+2])

		if seen[card] {
			printf("GRESKA\n")
			return
		}

		seen[card] = true
		suits[suit] -= 1
		i += 3
	}

	printf("%d %d %d %d\n", suits["P"], suits["K"], suits["H"], suits["T"])
}
