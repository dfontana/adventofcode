package main

import (
	"fmt"
	"log"

	"github.com/dfontana/adventofcode2019/util"
)

type ampRunner func(data []int, phases []int) int

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
	phases, best := searchPhases(data, 0, 5)
	fmt.Println("Part 1:", best, phases)

	phases, best = searchPhases(data, 5, 10)
	fmt.Println("Part 2:", best, phases)
}

func searchPhases(data []int, min int, max int) ([]int, int) {
	maxScore := 0
	var bestPhases []int

	for p1 := min; p1 < max; p1++ {
		for p2 := min; p2 < max; p2++ {
			if p2 == p1 {
				continue
			}
			for p3 := min; p3 < max; p3++ {
				if p3 == p1 || p3 == p2 {
					continue
				}
				for p4 := min; p4 < max; p4++ {
					if p4 == p3 || p4 == p2 || p4 == p1 {
						continue
					}
					for p5 := min; p5 < max; p5++ {
						if p5 == p4 || p5 == p3 || p5 == p2 || p5 == p1 {
							continue
						}
						tryThis := []int{p1, p2, p3, p4, p5}
						out := runAmp(data, tryThis)
						if out > maxScore {
							bestPhases = tryThis
							maxScore = out
						}
					}
				}
			}
		}
	}
	return bestPhases, maxScore
}

func runAmp(data []int, phases []int) int {
	feedback := make(chan int, 1)
	a := make(chan int)
	b := make(chan int)
	c := make(chan int)
	d := make(chan int)

	done := make(chan bool)

	go run(getMemory(data), feedback, a, done)
	go run(getMemory(data), a, b, done)
	go run(getMemory(data), b, c, done)
	go run(getMemory(data), c, d, done)
	go run(getMemory(data), d, feedback, done)

	feedback <- phases[0]
	a <- phases[1]
	b <- phases[2]
	c <- phases[3]
	d <- phases[4]

	feedback <- 0

	for i := 0; i < 5; i++ {
		<-done
	}

	return <-feedback
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

func run(memory []int, input <-chan int, output chan<- int, done chan<- bool) {
	ptr := 0
	maxAddr := len(memory)
	for {
		if ptr >= maxAddr {
			log.Fatal("About to segFault. Aborting...")
			done <- true
			return
		}

		op, modes := getOpModes(memory[ptr])
		paramCt := getParamCt(op)
		if paramCt >= maxAddr-ptr {
			log.Fatal("About to segFault. Aborting...")
			done <- true
			return
		}
		rawParams := getParams(memory, ptr, op)
		modeParams := applyModes(memory, modes, rawParams...)

		switch op {
		case opCodeAdd:
			memory[rawParams[2]] = modeParams[0] + modeParams[1]
		case opCodeMult:
			memory[rawParams[2]] = modeParams[0] * modeParams[1]
		case opCodeInput:
			memory[rawParams[0]] = <-input
		case opCodeOuput:
			output <- modeParams[0]
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
			done <- true
			return
		default:
			done <- true
			return
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
