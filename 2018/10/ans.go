package main

import (
	"fmt"

	"github.com/dfontana/adventofcode2019/util"
)

// Unit of one item in the sky
type Unit struct {
	x int
	y int
	vx int
	vy int
}

func main() {
	
}

func parseData(filename string) {
	lines := util.GetLines("./input.txt")
	for _, line := range lines {
		parsed := util.Split(line, "[=< ,>]+")
		unit := &Unit{
			x: 
		}
	}
}

