package main

import (
	"bufio"
	"fmt"
	"os"
)

var reader *bufio.Reader = bufio.NewReader(os.Stdin)
var writer *bufio.Writer = bufio.NewWriter(os.Stdout)
func printf(f string, a ...interface{}) { fmt.Fprintf(writer, f, a...) }
func scanf(f string, a ...interface{}) { fmt.Fscanf(reader, f, a...) }

func getThose(people map[string]int) (string, int, string, int) {
  least := 0
  max := 0
  var leastP, maxP string

  for key, value := range people {
    if value < least {
      leastP = key
      least = value
    }
    if value > max {
       maxP = key
       max = value
    }
  }

  return leastP, least, maxP, max
}


func minOf2(a int, b int) int {
   if a < b {
     return a
   } else {
     return b
   }
}

func main() {
  // STDOUT MUST BE FLUSHED MANUALLY!!!
  defer writer.Flush()

  people := make(map[string]int)
  var n int
  scanf("%d\n", &n)

  for i := 0; i < n; i++ {
    var p1, p2 string
    var drink int
    scanf("%s %s %d\n", &p1, &p2, &drink)

    person1, ok := people[p1]
    if !ok {
      person1 = 0
    }

    person2, ok := people[p2]
    if !ok {
      person2 = 0
    }

    people[p1] = person1 - drink
    people[p2] = person2 + drink
  }

  for true {
    leastP, least, maxP, max := getThose(people)
    if (least == max) {
      break
    }

    min := minOf2(-least, max)
    people[leastP] = least + min
    people[maxP] = max - min

    printf("%s %s %d\n", maxP, leastP, min)
  }
  printf("settled\n")
}

