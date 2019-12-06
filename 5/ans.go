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

func getOpModes(raw int) (int, []int) {
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

func getParams(memory []int, ptr int, op int) []int {
	ops := getParamCt(op)
	var params []int
	for i := 1; i <= ops; i++ {
		params = append(params, memory[ptr+i])
	}
	return params
}

func run(memory []int) (int, bool) {
	ptr := 0
	maxAddr := len(memory)
	for {
		if ptr >= maxAddr {
			log.Fatal("About to segFault. Aborting...")
			return 0, true
		}

		op, modes := getOpModes(memory[ptr])
		paramCt := getParamCt(op)
		if paramCt >= maxAddr-ptr {
			log.Fatal("About to segFault. Aborting...")
			return 0, true
		}
		rawParams := getParams(memory, ptr, op)
		modeParams := applyModes(memory, modes, rawParams...)

		switch op {
		case opCodeAdd:
			memory[rawParams[2]] = modeParams[0] + modeParams[1]
		case opCodeMult:
			memory[rawParams[2]] = modeParams[0] * modeParams[1]
		case opCodeInput:
			memory[rawParams[0]] = getInput()
		case opCodeOuput:
			fmt.Println(modeParams[0])
		case opCodeLessThan:
			if modeParams[0] < modeParams[1] {
				memory[rawParams[2]] = 1
			} else {
				memory[rawParams[2]] = 0
			}
		case opCodeEquals:
			if modeParams[0] == modeParams[1] {
				memory[rawParams[2]] = 1
			} else {
				memory[rawParams[2]] = 0
			}
		case opCodeJumpIfFalse:
			if modeParams[0] == 0 {
				ptr = modeParams[1]
				continue
			}
		case opCodeJumpIfTrue:
			if modeParams[0] != 0 {
				ptr = modeParams[1]
				continue
			}
		case opCodeAbort:
			return memory[0], true
		default:
			return 0, true
		}

		ptr += paramCt + 1
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
