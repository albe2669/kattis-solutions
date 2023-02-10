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

func getEven(n int) (int, int) {
  return 2, n/2
}

func main() {
  // STDOUT MUST BE FLUSHED MANUALLY!!!
  defer writer.Flush()

  var n int
  scanf("%d\n", &n)

  if n % 2 == 0 {
    a,b := getEven(n)
    printf("%dx%d\n", a, b)
  } else if n == 7 || n == 5 || n == 11 {
    printf("impossible\n")
  } else if n == 9 {
    printf("%dx%d\n", 3, 3)
  } else {
    for i := 2; i < (n/2); i++ {
      if n % i == 0 {
        printf("%dx%d\n", i, n/i)
        return
      }
    }
    a,b := getEven(n-9)
    printf("%dx%d %dx%d\n", 3, 3, a, b)
  }
}

