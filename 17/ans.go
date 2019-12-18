package main

import (
	"fmt"
	"strings"

	"github.com/dfontana/adventofcode2019/intcode"
)

func main() {
	data := intcode.ReadProgram("./input.txt")

	conf := intcode.Config()
	go intcode.Run(data, conf)

	fmt.Println("Part 1", alignmentStep(conf.Output))

	bufIn := make(chan int64, 100)
	conf2 := intcode.Config().SendDone().SetInput(bufIn)
	data[0] = 2
	go intcode.Run(data, conf2)

	fmt.Println("Part 2", traverseStep(conf2.Output, conf2.Input, conf2.Done))
}

func traverseStep(out <-chan int64, in chan<- int64, done <-chan bool) int64 {
	mmr := []string{"A", "B", "A", "B", "C", "B", "A", "C", "B", "C"}
	a := []string{"L", "12", "L", "8", "R", "10", "R", "10"}
	b := []string{"L", "6", "L", "4", "L", "12"}
	c := []string{"R", "10", "L", "8", "L", "4", "R", "10"}
	mmrStr := strings.Join(mmr, ",")
	aStr := strings.Join(a, ",")
	bStr := strings.Join(b, ",")
	cStr := strings.Join(c, ",")

	for _, v := range fmt.Sprintf("%s\n%s\n%s\n%s\nn\n", mmrStr, aStr, bStr, cStr) {
		in <- int64(v)
	}

	for {
		select {
		case item := <-out:
			if item > 128 {
				return item
			}
		case <-done:
			return -1
		}
	}
}

func alignmentStep(out <-chan int64) int {
	var space [][]rune
	var row []rune
	for {
		item := <-out
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
	return sum
}
