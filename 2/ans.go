package main

import (
	"fmt"
	"log"

	"github.com/dfontana/adventofcode2019/util"
)

const OP_CODE_ADD = 1
const OP_CODE_MULT = 2
const OP_CODE_ABORT = 99

func main() {
	data := parseInput("./input.txt")

	res, _ := run(getMemory(data))
	fmt.Println("Part 1:", res)

	target := 19690720
	res = findTarget(data, target)
	fmt.Println("Part 2:", res)
}

func findTarget(data []int, target int) int {
	for noun := 0; noun <= 99; noun++ {
		for verb := 0; verb <= 99; verb++ {
			memory := getMemory(data)
			memory[1], memory[2] = noun, verb
			res, _ := run(memory)
			if res == target {
				return 100*noun + verb
			}
		}
	}
	return -1
}

func getMemory(data []int) []int {
	memory := make([]int, len(data))
	copy(memory, data)
	return memory
}

func getParamCt(op int) int {
	switch op {
	case OP_CODE_ADD:
	case OP_CODE_MULT:
		return 3
	}
	return 0
}

func run(memory []int) (int, bool) {
	ptr := 0
	maxAddr := len(memory)
	for {
		if ptr >= maxAddr {
			log.Fatal("About to segFault. Aborting...")
			return 0, true
		}

		op := memory[ptr]

		if getParamCt(op) >= maxAddr-ptr {
			log.Fatal("About to segFault. Aborting...")
			return 0, true
		}

		switch op {
		case OP_CODE_ADD:
			x, y, ref := memory[ptr+1], memory[ptr+2], memory[ptr+3]
			memory[ref] = memory[x] + memory[y]
			ptr += 4
		case OP_CODE_MULT:
			x, y, ref := memory[ptr+1], memory[ptr+2], memory[ptr+3]
			memory[ref] = memory[x] * memory[y]
			ptr += 4
		case OP_CODE_ABORT:
			return memory[0], true
		default:
			return 0, true
		}
	}
}

func parseInput(filename string) []int {
	items := util.GetLines(filename)
	tokens := util.Split(items[0], ",")
	var result []int
	for _, token := range tokens {
		result = append(result, util.ToInt(token))
	}
	return result
}
