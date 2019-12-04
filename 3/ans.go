package main

import (
	"fmt"
	"math"

	"github.com/dfontana/adventofcode2019/util"
)

// Point represents an XY pair
type Point struct {
	X float64
	Y float64
}

func main() {
	data1, data2 := parseInput("./input.txt")
	wire1 := makeSegments(data1)
	wire2 := makeSegments(data2)

	var intersects []*Point
	for _, wire1s := range wire1 {
		for _, wire2s := range wire2 {
			p := intersect(wire1s[0], wire1s[1], wire2s[0], wire2s[1])
			if p != nil {
				intersects = append(intersects, p)
			}
		}
	}

	origin := Point{X: 0, Y: 0}
	min := 1000000.0
	var minP *Point
	for _, inter := range intersects {
		dist := manhatten(origin, *inter)
		if min > dist && dist != 0 {
			minP = inter
			min = dist
		}
	}
	fmt.Println(minP, min)
}

// manhatten distance of two points
func manhatten(a Point, b Point) float64 {
	return math.Abs(b.X-a.X) + math.Abs(b.Y-a.Y)
}

// determine intersection point of segments AB, CD
func intersect(a Point, b Point, c Point, d Point) *Point {
	Ax := b.X - a.X
	By := b.Y - a.Y
	Cx := d.X - c.X
	Dy := d.Y - c.Y

	deter := (Ax * Dy) - (Cx * By)

	if deter == 0 {
		// Collinear, which problem states isn't a case here
		return nil
	}

	s := (-By*(a.X-c.X) + Ax*(a.Y-c.Y)) / deter
	t := (Cx*(a.Y-c.Y) - Dy*(a.X-c.X)) / deter

	if s < 0 || s > 1 || t < 0 || t > 1 {
		return nil
	}

	// Find the intersection of the lines
	x := a.X + (t * Ax)
	y := a.Y + (t * By)

	// Check if it was within our segment
	if isBetween(a.X, x, b.X) && isBetween(a.Y, y, b.Y) {
		return &Point{X: x, Y: y}
	}

	// Otherwise we intersect, but just not in this segment
	return nil
}

// if Q is between P and R
func isBetween(p float64, q float64, r float64) bool {
	return q <= math.Max(p, r) && q >= math.Min(p, r)
}

// convert the array of instructions into workable points
func makeSegments(intrs []string) [][]Point {
	x := 0.0
	y := 0.0
	var wire [][]Point

	prior := Point{X: x, Y: y}

	for _, instr := range intrs {
		dir := string(instr[0])
		amount := float64(util.ToInt(string(instr[1:])))
		switch dir {
		case "R":
			x += amount
		case "L":
			x -= amount
		case "U":
			y += amount
		case "D":
			y -= amount
		}

		next := Point{X: x, Y: y}
		segment := []Point{prior, next}
		wire = append(wire, segment)
		prior = next
	}

	return wire
}

// extract lines from file, obtaining instructions for each
// wire that we need to expand.
func parseInput(filename string) ([]string, []string) {
	lines := util.GetLines(filename)
	instrs := util.Split(lines[0], ",")
	wire1 := make([]string, len(instrs))
	copy(wire1, instrs)

	instrs2 := util.Split(lines[1], ",")
	wire2 := make([]string, len(instrs2))
	copy(wire2, instrs2)

	return wire1, wire2
}
