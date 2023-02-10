package main

import (
	"fmt"
	"strings"
	"strconv"
)

func main() {
	var in string
	fmt.Scanln(&in)

	var split []string
	split = strings.Split(in, ";")
	
	var total = 0

	for _, s := range split {
		if strings.Contains(s, "-") {
			var splitAgain = strings.Split(s, "-")
			first,_ := strconv.Atoi(splitAgain[0])
			second,_ := strconv.Atoi(splitAgain[1])
			total += second - first + 1
		} else {
			total += 1
		}
	}

	fmt.Println(total)
}
