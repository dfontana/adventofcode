package main

import (
	"fmt"

	"github.com/dfontana/adventofcode2019/intcode"
)

func main() {
	data := intcode.ReadProgram("./input.txt")

	input, output, done := intcode.MakeComms()
	go intcode.Run(data, input, output, done)
	input <- 1

	fmt.Println("P1")
	intcode.PrintOut(output, done)

	go intcode.Run(data, input, output, done)
	input <- 2

	fmt.Println("P2")
	intcode.PrintOut(output, done)
}