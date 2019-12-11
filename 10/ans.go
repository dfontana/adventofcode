package main

import (
	"fmt"
	"math"

	"github.com/dfontana/adventofcode2019/util"
)

type coord struct {
	X int
	Y int
}

func (c coord) equals(other coord) bool {
	return other.X == c.X && other.Y == c.Y
}

func main() {
	data := parseInput("./input.txt")
	best, count := findMostVisible(data)
	fmt.Println("P1:", best, "Sees", count, "out of", len(data))
}

// 307 too high. 292 too low. Hermph.
func findMostVisible(asteroids []coord) (coord, int) {
	maxCt := 0
	var bestCoord coord
	for _, origin := range asteroids {
		myCt := 0
		for _, dest := range asteroids {
			if origin.equals(dest) {
				continue
			}
			if !blocked(asteroids, origin, dest) {
				myCt++
			}
		}
		if myCt > maxCt {
			maxCt = myCt
			bestCoord = origin
		}
	}
	return bestCoord, maxCt
}

func blocked(asteroids []coord, origin coord, dest coord) bool {
	for _, asteroid := range asteroids {
		if asteroid.equals(origin) || asteroid.equals(dest) {
			continue
		}
		if inLineOfSight(origin, asteroid, dest) {
			return true
		}
	}
	return false
}

func inLineOfSight(a, c, b coord) bool {
	crossproduct := float64(c.Y-a.Y)*float64(b.X-a.X) - float64(c.X-a.X)*float64(b.Y-a.Y)
	if math.Abs(crossproduct) > 0.0000001 {
		return false
	}

	dotproduct := float64(c.X-a.X)*float64(b.X-a.X) + float64(c.Y-a.Y)*float64(b.Y-a.Y)
	if dotproduct < 0 {
		return false
	}

	squaredlengthba := float64(b.X-a.X)*float64(b.X-a.X) + float64(b.Y-a.Y)*float64(b.Y-a.Y)
	if dotproduct > squaredlengthba {
		return false
	}

	return true
}

func parseInput(filename string) []coord {
	lines := util.GetLines(filename)
	var coords []coord
	for y, line := range lines {
		for x, space := range line {
			if space == '#' {
				coords = append(coords, coord{X: x, Y: y})
			}
		}
	}
	return coords
}
