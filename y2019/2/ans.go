package main

import (
	"fmt"

	"github.com/dfontana/adventofcode/2019/intcode"
)

func main() {
	data := intcode.ReadProgram("./input.txt")

	res := intcode.Run(data, intcode.Config())
	fmt.Println("Part 1:", res)

	target := int64(19690720)
	res2 := findTarget(data, target)
	fmt.Println("Part 2:", res2)
}

func findTarget(data []int64, target int64) int {
	for noun := 0; noun <= 99; noun++ {
		for verb := 0; verb <= 99; verb++ {
			memory := intcode.GetMemory(data)
			memory[1], memory[2] = int64(noun), int64(verb)
			res := intcode.Run(memory, intcode.Config())
			if res == target {
				return 100*noun + verb
			}
		}
	}
	return -1
}
