package main

import (
	"fmt"

	"github.com/dfontana/adventofcode2019/intcode"
)

const empty = 0
const wall = 1
const block = 2
const hPaddle = 3
const ball = 4

type tile struct {
	x  int
	y  int
	id int
}

func main() {
	data := intcode.ReadProgram("./input.txt")
	input, output, done := intcode.MakeComms()

	go intcode.Run(data, input, output, done)

	var tiles []tile
	instr := []int{0, 0, 0}

	i := 0
	stop := false
	for !stop {
		select {
		case val, ok := <-output:
			if ok {
				instr[i%3] = int(val)
			}
		case <-done:
			stop = true
		}

		if (i+1)%3 == 0 {
			// Flush the instr
			tiles = append(tiles, tile{instr[0], instr[1], instr[2]})
		}

		i++
	}

	ct := 0
	for _, t := range tiles {
		if t.id == block {
			ct++
		}
	}
	fmt.Println("Part 1", ct)
}
