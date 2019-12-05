package main

import (
	"fmt"
	"strconv"
)

func main() {
	passes := 0
	for i := 130254; i <= 678275; i++ {
		if isPassword(i) {
			passes++
		}
	}
	fmt.Println("Passes:", passes, isPassword(111111), isPassword(223450), isPassword(123789), isPassword(112233), isPassword(123444), isPassword(111122))
}

func isPassword(val int) bool {
	valStr := strconv.FormatInt(int64(val), 10)
	var digits []int
	for _, char := range valStr {
		digits = append(digits, int(char))
	}

	prior := -1
	twoSame := false
	noIncrease := true
	tooLong := false
	hasSafeRun := false
	longestRun := 1
	for i, digit := range digits {
		matched := digit == prior
		if matched {
			twoSame = true
			longestRun++
			if longestRun == 3 {
				tooLong = true
			}
		}

		if !matched || i == len(digits)-1 {
			if longestRun == 2 {
				hasSafeRun = true
			}
			longestRun = 1
		}

		if digit < prior {
			noIncrease = false
			break
		}
		prior = digit
	}

	return twoSame && noIncrease && (!tooLong || hasSafeRun)
}
