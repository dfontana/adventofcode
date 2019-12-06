package main

import (
	"bufio"
	"fmt"
	"log"
	"os"

	"github.com/dfontana/adventofcode2019/util"
)

const opCodeAdd = 1
const opCodeMult = 2
const opCodeAbort = 99
const opCodeInput = 3
const opCodeOuput = 4
const opCodeJumpIfTrue = 5
const opCodeJumpIfFalse = 6
const opCodeLessThan = 7
const opCodeEquals = 8

func main() {
	data := parseInput("./input.txt")
	res, _ := run(getMemory(data))
	fmt.Println("Part 1:", res)
}

// Blindly takes input from command line and assumes its an integer.
func getInput() int {
	reader := bufio.NewReader(os.Stdin)
	fmt.Print("Enter Input: ")
	text, _ := reader.ReadString('\n')
	return util.ToInt(text[:len(text)-1])
}

func getMemory(data []int) []int {
	memory := make([]int, len(data))
	copy(memory, data)
	return memory
}

func getParamCt(op int) int {
	switch op {
	case opCodeJumpIfFalse:
		fallthrough
	case opCodeJumpIfTrue:
		return 2
	case opCodeLessThan:
		fallthrough
	case opCodeEquals:
		fallthrough
	case opCodeAdd:
		fallthrough
	case opCodeMult:
		return 3
	case opCodeInput:
		fallthrough
	case opCodeOuput:
		return 1
	}
	return 0
}

func getOpParams(raw int) (int, []int) {
	modes := []int{0, 0, 0}
	op := 0
	for i := 0; raw > 0; i++ {
		val := raw % 10
		switch i {
		case 0:
			op += val
		case 1:
			op += 10 * val
		default:
			modes[i-2] = val
		}
		raw /= 10
	}
	return op, modes
}

func applyModes(memory []int, modes []int, params ...int) []int {
	var items []int
	for i, param := range params {
		modeParam := param
		if modes[i] == 0 {
			modeParam = memory[param]
		}
		items = append(items, modeParam)
	}
	return items
}

func run(memory []int) (int, bool) {
	ptr := 0
	maxAddr := len(memory)
	for {
		if ptr >= maxAddr {
			log.Fatal("About to segFault. Aborting...")
			return 0, true
		}

		op, modes := getOpParams(memory[ptr])
		if getParamCt(op) >= maxAddr-ptr {
			log.Fatal("About to segFault. Aborting...")
			return 0, true
		}

		switch op {
		case opCodeAdd:
			x, y, ref := memory[ptr+1], memory[ptr+2], memory[ptr+3]
			params := applyModes(memory, modes, x, y)
			memory[ref] = params[0] + params[1]
			ptr += 4
		case opCodeMult:
			x, y, ref := memory[ptr+1], memory[ptr+2], memory[ptr+3]
			params := applyModes(memory, modes, x, y)
			memory[ref] = params[0] * params[1]
			ptr += 4
		case opCodeInput:
			ref := memory[ptr+1]
			memory[ref] = getInput()
			ptr += 2
		case opCodeOuput:
			ref := memory[ptr+1]
			params := applyModes(memory, modes, ref)
			fmt.Println(params[0])
			ptr += 2
		case opCodeLessThan:
			x, y, ref := memory[ptr+1], memory[ptr+2], memory[ptr+3]
			params := applyModes(memory, modes, x, y)
			if params[0] < params[1] {
				memory[ref] = 1
			} else {
				memory[ref] = 0
			}
			ptr += 4
		case opCodeEquals:
			x, y, ref := memory[ptr+1], memory[ptr+2], memory[ptr+3]
			params := applyModes(memory, modes, x, y)
			if params[0] == params[1] {
				memory[ref] = 1
			} else {
				memory[ref] = 0
			}
			ptr += 4
		case opCodeJumpIfFalse:
			x, y := memory[ptr+1], memory[ptr+2]
			params := applyModes(memory, modes, x, y)
			if params[0] == 0 {
				ptr = params[1]
				continue
			}
			ptr += 3
		case opCodeJumpIfTrue:
			x, y := memory[ptr+1], memory[ptr+2]
			params := applyModes(memory, modes, x, y)
			if params[0] != 0 {
				ptr = params[1]
				continue
			}
			ptr += 3
		case opCodeAbort:
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
