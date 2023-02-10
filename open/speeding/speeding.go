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
	
	var lastDistance, lastTime, fastestSpeed int
	scanf("%d %d\n", &lastDistance, &lastTime)
	
	for i := 1; i < n; i++ {
		var distance, time, speed int
		scanf("%d %d\n", &distance, &time)
		speed = (time - lastTime) / (distance - lastDistance)

		if speed > fastestSpeed {
			fastestSpeed = speed
		}

		lastTime = time
		lastDistance = distance
	}

	printf("%d\n", fastestSpeed)
}
