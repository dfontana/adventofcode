package main

import (
	"fmt"

	"github.com/dfontana/adventofcode2019/intcode"
)

var prog []int64

func main() {
	prog = intcode.ReadProgram("./input.txt")

	board := make([][]int, 50)
	for i := range board {
		board[i] = make([]int, 50)
	}

	for r := 0; r < 50; r++ {
		for c := 0; c < 50; c++ {
			board[r][c] = testSpace(r, c)
		}
	}

	totalAffected := 0
	for r := 0; r < 50; r++ {
		for c := 0; c < 50; c++ {
			if testSpace(r, c) == 1 {
				fmt.Print("#")
				totalAffected++
			} else {
				fmt.Print(".")
			}
		}
		fmt.Println()
	}

	fmt.Println("P1", totalAffected)

	tX, tY := 0, 0
	beamStart := false
	for {
		pInBeam := testSpace(tX, tY) == 1
		if !pInBeam && !beamStart {
			// Point is left of the beam
			tX++
			continue
		} else if !pInBeam && beamStart {
			// We've left the beam
			tY++
			tX = 0
			beamStart = false
			continue
		} else if pInBeam && !beamStart {
			// First point in the beam
			beamStart = true
		}

		// We're inside the beam, so let's test if the full 100x100 fits
		if testSpace(tX+99, tY) == 1 && testSpace(tX, tY+99) == 1 && testSpace(tX+99, tY+99) == 1 {
			// We found it
			fmt.Println("P2", tX*10000+tY)
			return
		}

		tX++
	}
}

func testSpace(x, y int) int {
	in := make(chan int64, 2)
	in <- int64(x)
	in <- int64(y)
	conf := intcode.Config().SendDone().SetInput(in)
	go intcode.Run(prog, conf)
	return int(<-conf.Output)
}
