package main

import (
	"fmt"

	"github.com/dfontana/adventofcode2019/intcode"
)

func main() {
	data := intcode.ReadProgram("./input.txt")
	phases, best := searchPhases(data, 0, 5)
	fmt.Println("Part 1:", best, phases)

	phases, best = searchPhases(data, 5, 10)
	fmt.Println("Part 2:", best, phases)
}

func searchPhases(data []int64, min int64, max int64) ([]int64, int64) {
	maxScore := int64(0)
	var bestPhases []int64

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
						tryThis := []int64{p1, p2, p3, p4, p5}
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

func runAmp(data []int64, phases []int64) int64 {
	feedback := make(chan int64, 1)
	a := make(chan int64)
	b := make(chan int64)
	c := make(chan int64)
	d := make(chan int64)

	go intcode.Run(data, feedback, a)
	go intcode.Run(data, a, b)
	go intcode.Run(data, b, c)
	go intcode.Run(data, c, d)
	go intcode.Run(data, d, feedback)

	feedback <- phases[0]
	a <- phases[1]
	b <- phases[2]
	c <- phases[3]
	d <- phases[4]

	feedback <- 0
	return <-feedback
}
