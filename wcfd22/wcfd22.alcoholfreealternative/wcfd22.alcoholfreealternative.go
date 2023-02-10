package main

import (
  "bufio"
  "fmt"
)

var reader *bufio.Reader = bufio.NewReader(os.Stdin)
var writer *bufio.Writer = bufio.NewWriter(os.Stdout)
func printf(f string, a ...interface{}) { fmt.Fprintf(writer, f, a...) }
func scanf(f string, a ...interface{}) { fmt.Fscanf(reader, f, a...) }

func getDrink(drinkMap map[string]int) string {
  var maxKey string
  var maxVal := 0

  for key, elem := range drinkMap {
    if elem > 0 {
      maxVal = elem
      maxKey = key
    }
  }

  return maxKey
}

func increaseMap(drinkMap map[string]int, drink string) {
  var val, ok := drinkMap[drink]
  if !ok {
    val = 0
  }

  drinkMap[drink] = val + 1
}

func main() {
  // STDOUT MUST BE FLUSHED MANUALLY!!!
  defer writer.Flush()

  var beers = make(map[string]int)
  var alcFree = make(map[string]int)

  var n int
  scanf("%d\n", &n)

  for i := 0; i < n; i++ {
    var beer, nonAlc string
    scanf("%s %s\n", &beer, &nonAlc)
    printf("Read: '%s' '%s'\n", beer, nonAlc)

    increaseMap(beers, beer)
    increaseMap(alcFree, nonAlc)
  }

  printf("%s %\n", a+b)
}

