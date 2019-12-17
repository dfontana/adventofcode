package main

import (
	"fmt"
	"math"

	"github.com/dfontana/adventofcode2019/util"
)

func main() {
	clean := parseInput("./input.txt")

	signal := make([]int, len(clean))
	copy(signal, clean)
	for i := 0; i < 100; i++ {
		signal = phase(signal)
	}

	fmt.Println("P1:", signal[0:8])

	var signal2 []int
	for i := 0; i < 10000; i++ {
		signal2 = append(signal2, clean...)
	}

	offset := concat(clean[:7])
	signal2 = signal2[offset:]
	for i := 0; i < 100; i++ {
		acc, ct := 0, len(signal2)
		for index := ct - 1; index >= 0; index-- {
			acc += int(signal2[index])
			signal2[index] = int(math.Abs(float64(acc % 10)))
		}
	}

	fmt.Println("P2:", signal2[0:8])
}

func concat(items []int) int {
	factor, digits := int(math.Pow(float64(10), float64(len(items)-1))), len(items)
	sum := 0
	for i := 0; i < digits; i++ {
		if factor == 0 {
			sum += items[i]
			break
		}
		sum += items[i] * factor
		factor /= 10
	}
	return sum
}

func phase(signal []int) []int {
	var newSignal []int
	total := len(signal)
	seedPattern := []int{0, 1, 0, -1}

	pattern := make([]int, 4)
	copy(pattern, seedPattern)

	for ele := 0; ele < total; ele++ {
		// Get the bit for the new signal
		sum, idx, mod := 0, 1, len(pattern)
		for _, bit := range signal {
			sum += bit * pattern[idx%mod]
			idx++
		}

		newSignal = append(newSignal, int(math.Abs(float64(sum)))%10)

		// Update pattern with repeating digits
		var newPattern []int
		prev, ptr := pattern[0], 0
		for ; ptr < mod; ptr++ {
			if prev != pattern[ptr] {
				// Append a new copy of the prev
				newPattern = append(newPattern, prev)
				prev = pattern[ptr]
			}
			newPattern = append(newPattern, pattern[ptr])
		}
		newPattern = append(newPattern, pattern[ptr-1])
		pattern = newPattern
	}
	return newSignal
}

func parseInput(filename string) []int {
	lines := util.GetLines(filename)
	var signal []int
	for _, item := range lines[0] {
		signal = append(signal, util.ToInt(string(item)))
	}
	return signal
}
