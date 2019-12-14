package main

import (
	"fmt"

	"github.com/dfontana/adventofcode2019/intcode"
)

func main() {
	data := intcode.ReadProgram("./input.txt")
	input, output := intcode.MakeComms()
	go intcode.Run(data, input, output)
	input <- 1
	fmt.Print("Part 1 (first non-zero): ")
	intcode.PrintOut(output)

	input, output = intcode.MakeComms()
	go intcode.Run(data, input, output)
	input <- 5
	fmt.Print("Part 2 (first non-zero): ")
	intcode.PrintOut(output)
}
