package main

import (
	"fmt"
	"log"

	"github.com/dfontana/adventofcode2019/util"
)

func main() {
	data := parseInput("./input.txt")
	target := 19690720
	res := findTarget(data, target)
	fmt.Println("Result:", res)
}

func findTarget(data []int, target int) int {
	for noun := 0; noun <= 99; noun++ {
		for verb := 0; verb <= 99; verb++ {
			memory := getMemory(data)
			memory[1] = noun
			memory[2] = verb
			runProgram(memory)
			if memory[0] == target {
				return 100*noun + verb
			}
		}
	}
	return -1
}

func runProgram(memory []int) {
	tokens := len(memory)
	for i := 0; i < tokens; i += 4 {
		inst := memory[i]
		x := memory[safeAddress(i+1, memory, tokens)]
		y := memory[safeAddress(i+2, memory, tokens)]
		ref := safeAddress(i+3, memory, tokens)
		res, done := operate(inst, x, y)
		if done {
			return
		}
		memory[ref] = res
	}
}

func getMemory(data []int) []int {
	var clean []int
	for _, i := range data {
		clean = append(clean, i)
	}
	return clean
}

func safeAddress(idx int, data []int, length int) int {
	if idx >= length {
		return 0
	}
	ref := data[idx]
	if ref >= length {
		return 0
	}
	return ref
}

func operate(op int, x int, y int) (int, bool) {
	switch op {
	case 1:
		return x + y, false
	case 2:
		return x * y, false
	case 99:
		return 0, true
	default:
		log.Fatal("Unknown op code hit")
		return 0, true
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
