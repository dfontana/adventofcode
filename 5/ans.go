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
	fmt.Print("Part 1 (first non-zero): ")
	intcode.PrintOut(output, done)

	go intcode.Run(data, input, output, done)
	input <- 5
	fmt.Print("Part 2 (first non-zero): ")
	intcode.PrintOut(output, done)
}