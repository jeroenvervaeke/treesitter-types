package main

import "fmt"

type Point struct {
    X int
    Y int
}

func greet(name string) string {
    return "Hello, " + name
}

func main() {
    fmt.Println(greet("world"))
}
