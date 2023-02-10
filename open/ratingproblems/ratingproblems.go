package main

import (
	"fmt"
)

func main() {
	var all, toGet int
	fmt.Scanln(&all, &toGet)

	var total float32 = 0

	for i := 0; i < toGet; i++ {
		var a float32
		fmt.Scanln(&a)

		total += a
	}

	var allFloat float32 = float32(all)
	var lo float32 = (float32(all - toGet) * -3) + total
	var hi float32 = (float32(all - toGet) * 3) + total

	fmt.Println(lo / allFloat, hi / allFloat)
}
