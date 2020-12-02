package main

import "fmt"

type runChecker func(int) bool

func main() {
	part1 := checkRange(130254, 678275, func(r int) bool { return r >= 2 })
	part2 := checkRange(130254, 678275, func(r int) bool { return r == 2 })
	fmt.Println("Passes:", part1, part2)
}

func checkRange(min int, max int, runCheck runChecker) int {
	passwords := 0
	for i := min; i <= max; i++ {
		if isPassword(i, runCheck) {
			passwords++
		}
	}
	return passwords
}

func breakDigits(val int) []int {
	var digits []int
	for val > 0 {
		digits = append(digits, val%10)
		val /= 10
	}

	var result []int
	for i := len(digits) - 1; i >= 0; i-- {
		result = append(result, digits[i])
	}
	return result
}

func isPassword(val int, runCheck runChecker) bool {
	digits := breakDigits(val)

	prior := -1
	run := 1
	hasSafeRun := false

	end := len(digits) - 1
	for i, digit := range digits {
		if digit < prior {
			return false
		}

		matched := digit == prior
		if matched {
			run++
		}

		if !matched || i == end {
			if runCheck(run) {
				hasSafeRun = true
			}
			run = 1
		}

		prior = digit
	}

	return hasSafeRun
}
