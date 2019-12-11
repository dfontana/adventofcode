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

func (c *coord) add(x, y int) {
	 c.X = c.X + x
	 c.Y = c.Y + y
}

func main() {
	asteroids, fieldWidth, fieldHeight := parseInput("./input.txt")
	best, count := findMostVisible(asteroids)
	fmt.Println("P1:", best, "Sees", count, "out of", len(asteroids))

	nth := 200
	lastToDie := findNthAnnihilated(asteroids, best, nth, fieldWidth, fieldHeight)
	fmt.Println("P2:", nth, "th asteroid to vaporize is", lastToDie, "aka", lastToDie.X*100+lastToDie.Y)
}

func findNthAnnihilated(asteroids []coord, origin coord, nth, width, height int) coord {
	vaporCount := 1
	dest := coord{X: origin.X, Y: 0}
	for {
		// Find the asteroid to vaporize, if any
		shortestDist := 1000000000.0
		var idxOfAsteroidToBoom int

		for idx, asteroid := range asteroids {
			if asteroid.equals(origin) {
				continue
			}
			if (inLineOfSight(origin, asteroid, dest)) {
				dist := distSquared(origin, asteroid)
				if (dist < shortestDist) {
					shortestDist = dist
					idxOfAsteroidToBoom = idx
				}
			}
		}

		// Boom the asteroid
		if vaporCount == nth {
			return asteroids[idxOfAsteroidToBoom]
		}
		asteroids = append(asteroids[:idxOfAsteroidToBoom], asteroids[idxOfAsteroidToBoom+1:]...)
		vaporCount++

		// Rotate the dest pointer 1
		if dest.X + 1 < width && dest.Y == 0 {
			dest.add(1, 0)
		} else if dest.Y + 1 < height && dest.X == width-1 {
			dest.add(0, 1)
		} else if dest.X - 1 > -1 && dest.Y == height-1{
			dest.add(-1, 0)
		} else {
			dest.add(0, -1)
		}
	}
}

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

func distSquared(a, b coord) float64 {
	return math.Pow(float64(b.X-a.X), 2) + math.Pow(float64(b.Y-a.Y), 2)
}

func parseInput(filename string) ([]coord, int, int) {
	lines := util.GetLines(filename)
	var width int
	var coords []coord
	for y, line := range lines {
		width = len(line)
		for x, space := range line {
			if space == '#' {
				coords = append(coords, coord{X: x, Y: y})
			}
		}
	}
	return coords, width, len(lines)
}
