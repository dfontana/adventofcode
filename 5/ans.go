package main

import (
	"fmt"

	"github.com/dfontana/adventofcode2019/intcode"
)

func main() {
	data := intcode.ReadProgram("./input.txt")
	config := intcode.Config()
	go intcode.Run(data, config)
	config.Input <- 1
	fmt.Print("Part 1 (first non-zero): ")
	intcode.PrintOut(config.Output)

	config = intcode.Config()
	go intcode.Run(data, config)
	config.Input <- 5
	fmt.Print("Part 2 (first non-zero): ")
	intcode.PrintOut(config.Output)
}
