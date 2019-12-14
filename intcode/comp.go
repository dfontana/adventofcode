package intcode

import (
	"fmt"

	"github.com/dfontana/adventofcode2019/util"
)

const opCodeAdd = 1
const opCodeMult = 2
const opCodeInput = 3
const opCodeOuput = 4
const opCodeJumpIfTrue = 5
const opCodeJumpIfFalse = 6
const opCodeLessThan = 7
const opCodeEquals = 8
const opCodeRelativeBase = 9
const opCodeAbort = 99

// PrintOut coming from the computer
func PrintOut(output <-chan int64) {
	for val := range output {
		fmt.Print(val, ",")
	}
	fmt.Println("")
}

// GetMemory for safe cloning
func GetMemory(data []int64) []int64 {
	memory := make([]int64, len(data))
	copy(memory, data)
	return memory
}

// Run the intcode computer
func Run(data []int64, conf config) int64 {
	memory := GetMemory(data)
	var ptr, relativeBase int64

	getPtr := func(ptr int64) *int64 {
		for int64(len(memory)) <= ptr {
			memory = append(memory, 0)
		}
		return &memory[ptr]
	}

	for {
		op, mode := getOpModes(memory[ptr])

		getParam := func(mode int64, offset int64) *int64 {
			param := memory[ptr+offset]
			switch mode {
			case 0:
				return getPtr(param)
			case 1:
				return &param
			case 2:
				return getPtr(relativeBase + param)
			default:
				panic("Unknown mode")
			}
		}

		switch op {
		case opCodeAdd:
			x, y, ref := getParam(mode[0], 1), getParam(mode[1], 2), getParam(mode[2], 3)
			*ref = *x + *y
		case opCodeMult:
			x, y, ref := getParam(mode[0], 1), getParam(mode[1], 2), getParam(mode[2], 3)
			*ref = *x * *y
		case opCodeInput:
			ref := getParam(mode[0], 1)
			if conf.sendRequestSignal {
				conf.Request <- true
			}
			*ref = <-conf.Input
		case opCodeOuput:
			conf.Output <- *(getParam(mode[0], 1))
		case opCodeLessThan:
			x, y, ref := getParam(mode[0], 1), getParam(mode[1], 2), getParam(mode[2], 3)
			if *x < *y {
				*ref = 1
			} else {
				*ref = 0
			}
		case opCodeEquals:
			x, y, ref := getParam(mode[0], 1), getParam(mode[1], 2), getParam(mode[2], 3)
			if *x == *y {
				*ref = 1
			} else {
				*ref = 0
			}
		case opCodeJumpIfFalse:
			x, y := getParam(mode[0], 1), getParam(mode[1], 2)
			if *x == 0 {
				ptr = *y
				continue
			}
		case opCodeJumpIfTrue:
			x, y := getParam(mode[0], 1), getParam(mode[1], 2)
			if *x != 0 {
				ptr = *y
				continue
			}
		case opCodeRelativeBase:
			x := getParam(mode[0], 1)
			relativeBase += *x
		case opCodeAbort:
			close(conf.Output)
			if conf.sendDoneSignal {
				conf.Done <- true
			}
			return memory[0]
		default:
			panic("Unknown Op Code")
		}

		ptr += getParamCt(op) + 1
	}
}

// ReadProgram from disk
func ReadProgram(filename string) []int64 {
	items := util.GetLines(filename)
	tokens := util.Split(items[0], ",")
	var result []int64
	for _, token := range tokens {
		result = append(result, util.ToInt64(token))
	}
	return result
}

func getParamCt(op int64) int64 {
	ct := 0
	switch op {
	case opCodeJumpIfFalse:
		fallthrough
	case opCodeJumpIfTrue:
		ct = 2
	case opCodeLessThan:
		fallthrough
	case opCodeEquals:
		fallthrough
	case opCodeAdd:
		fallthrough
	case opCodeMult:
		ct = 3
	case opCodeRelativeBase:
		fallthrough
	case opCodeInput:
		fallthrough
	case opCodeOuput:
		ct = 1
	}
	return int64(ct)
}

func getOpModes(raw int64) (int64, []int64) {
	modes := []int64{0, 0, 0}
	op := int64(0)
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
