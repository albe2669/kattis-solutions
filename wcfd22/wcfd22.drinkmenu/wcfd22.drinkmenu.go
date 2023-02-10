package main

import (
  "bufio"
  "fmt"
  "os"
  "strings"
)

var reader *bufio.Reader = bufio.NewReader(os.Stdin)
var writer *bufio.Writer = bufio.NewWriter(os.Stdout)
func printf(f string, a ...interface{}) { fmt.Fprintf(writer, f, a...) }
func scanf(f string, a ...interface{}) { fmt.Fscanf(reader, f, a...) }
func readline() string {
  line, _ := reader.ReadString('\n')
  return strings.TrimSpace(line)
}

func main() {
  // STDOUT MUST BE FLUSHED MANUALLY!!!
  defer writer.Flush()
 
  var n, m int
  scanf("%d %d\n", &n, &m)

  patrons := make(map[string]int)
  drinks := make([]string, n)

  for i := 0; i < n; i++ {
    drinks[i] = readline()
  }

  for i := 0; i < m; i++ {
    patron := readline()

    patronN, ok := patrons[patron]
    if !ok {
      patronN = -1
    }

    patronN += 1

    printf("%s\n", drinks[patronN])

    patrons[patron] = patronN
  }
}

