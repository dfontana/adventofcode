package main

import (
	"fmt"
	"math"

	"github.com/dfontana/adventofcode/2019/util"
)

// Unit of one item in the sky
type Unit struct {
	x  int
	y  int
	vx int
	vy int
}

func (u *Unit) getX(step int) int {
	return u.x + u.vx*step
}

func (u *Unit) getY(step int) int {
	return u.y + u.vy*step
}

type Bound struct {
	x1, y1, x2, y2 int
}

func (b *Bound) getW() int {
	return int(math.Abs(float64(b.x2 - b.x1)))
}

func (b *Bound) getH() int {
	return int(math.Abs(float64(b.y2 - b.y1)))
}

func (b *Bound) LessThan(o *Bound) bool {
	myA := b.getW() * b.getH()
	oA := o.getW() * o.getH()
	return oA > myA
}

func main() {
	data := parseData("./input.txt")
	minB, s := runSim(data, 10946, 100000)
	fmt.Println(minB, s)
	printSim(data, s)
}

func printSim(coords []Unit, step int) {
	matrix := make([][]bool, 300)
	for y := 0; y < 300; y++ {
		matrix[y] = make([]bool, 300)
	}
	for _, coord := range coords {
		myX := coord.getX(step)
		myY := coord.getY(step)
		if myX < 300 && myY < 300 {
			matrix[myY][myX] = true
		}
	}
	for i := 185; i < 200; i++ {
		for j := 135; j < 210; j++ {
			if matrix[i][j] == true {
				fmt.Print("#")
			} else {
				fmt.Print("_")
			}
		}
		fmt.Println("")
	}
}

func runSim(coords []Unit, stepStart int, stepMax int) (*Bound, int) {
	step := stepStart
	minB := &Bound{x1: -10000, y1: -10000, x2: 10000, y2: 10000}
	minStep := stepStart
	for ; step < stepMax; step++ {
		b := bound(coords, step)
		if b.LessThan(minB) {
			minB = b
			minStep = step
		}
	}
	return minB, minStep
}

func bound(coords []Unit, step int) *Bound {
	minX := 0
	minY := 0
	maxX := 0
	maxY := 0
	for _, coord := range coords {
		simX := coord.getX(step)
		simY := coord.getY(step)
		if simX < minX {
			minX = simX
		}
		if simY < minY {
			minY = simY
		}
		if simX > maxX {
			maxX = simX
		}
		if simY > maxY {
			maxY = simY
		}
	}
	return &Bound{x1: minX, y1: minY, x2: maxX, y2: maxY}
}

func parseData(filename string) []Unit {
	lines := util.GetLines(filename)
	units := make([]Unit, len(lines))
	for _, line := range lines {
		parsed := util.Split(line, "[=< ,>]+")
		unit := Unit{
			x:  util.ToInt(parsed[1]),
			y:  util.ToInt(parsed[2]),
			vx: util.ToInt(parsed[4]),
			vy: util.ToInt(parsed[5]),
		}
		units = append(units, unit)
	}
	return units
}
