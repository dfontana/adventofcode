package main

import (
	"fmt"

	"github.com/dfontana/adventofcode2019/intcode"
)

func main() {
	data := intcode.ReadProgram("./input.txt")

	conf := intcode.Config()
	go intcode.Run(data, conf)

	var space [][]rune
	var row []rune
	for {
		item := <-conf.Output
		if item == 0 {
			space = space[0 : len(space)-1]
			break
		}
		runed := rune(item)
		switch runed {
		case '\n':
			space = append(space, row)
			row = nil
		default:
			row = append(row, runed)
		}
	}

	var intersects [][]int

	totalRows, totalCols := len(space), len(space[0])
	for row, r := range space {
		isTop, isBot := row == 0, row == totalRows-1
		for col, c := range r {
			fmt.Print(string(c))
			if c != '#' {
				continue
			}
			isLeft, isRight := col == 0, col == totalCols-1

			if isTop || isBot || isLeft || isRight {
				// Cant have intersect in 2 or 3 spots, must be 4 by definition
				// of this challenge....
				continue
			}

			if space[row+1][col] == '#' && space[row-1][col] == '#' && space[row][col+1] == '#' && space[row][col-1] == '#' {
				// Intersect
				intersect := []int{row, col}
				intersects = append(intersects, intersect)
			}
		}
		fmt.Println()
	}

	// Do the computation
	sum := 0
	for _, i := range intersects {
		sum += i[0] * i[1]
	}
	fmt.Println("Part 1", sum)
}
