package main

import (
	"fmt"

	"github.com/dfontana/adventofcode2019/intcode"
)

func main() {
	data := intcode.ReadProgram("./input.txt")

	conf := intcode.Config()
	go intcode.Run(data, conf)
	conf.Input <- 1

	fmt.Println("P1")
	intcode.PrintOut(conf.Output)

	conf = intcode.Config()
	go intcode.Run(data, conf)
	conf.Input <- 2

	fmt.Println("P2")
	intcode.PrintOut(conf.Output)
}
