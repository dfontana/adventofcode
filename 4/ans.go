package main

import (
	"fmt"
	"strconv"
)

func main() {
	passes := 0
	for i := 130254; i <= 678275; i++ {
		if isPassword(int64(i)) {
			passes++
		}
	}
	fmt.Println("Passes:", passes)
}

func isPassword(val int64) bool {
	valStr := strconv.FormatInt(val, 10)
	var digits []int
	for _, char := range valStr {
		digits = append(digits, int(char))
	}

	prior := -1
	twoSame := false
	noIncrease := true
	for _, digit := range digits {
		if digit == prior {
			twoSame = true
		}
		if digit < prior {
			noIncrease = false
			break
		}
		prior = digit
	}

	return twoSame && noIncrease
}
