package main

import (
	"fmt"
	"math"

	"github.com/dfontana/adventofcode/2019/util"
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
	pX, pY := float64(origin.X), 0.0
	delta := 0.0001
	for {
		// Find the asteroid to vaporize
		var idxOfAsteroidToBoom int
		smallestAngle := 1000.0

		for idx, asteroid := range asteroids {
			if asteroid.equals(origin) {
				continue
			}
			if !blocked(asteroids, origin, asteroid) {
				angle := getAngle(origin, pX, pY, asteroid)
				if angle >= 0 && angle < smallestAngle {
					smallestAngle = angle
					idxOfAsteroidToBoom = idx
				}
			}
		}

		// Boom the asteroid
		boom := asteroids[idxOfAsteroidToBoom]
		pX = float64(boom.X)
		pY = float64(boom.Y)
		if vaporCount == nth {
			return boom
		}
		asteroids = append(asteroids[:idxOfAsteroidToBoom], asteroids[idxOfAsteroidToBoom+1:]...)
		vaporCount++

		// Rotate the target pointer slightly, so we dont vaporize in a line
		pX = float64(origin.X) + (pX-float64(origin.X))*math.Cos(delta) - (pY-float64(origin.Y))*math.Sin(delta)
		pY = float64(origin.Y) + (pX-float64(origin.X))*math.Sin(delta) + (pY-float64(origin.Y))*math.Cos(delta)
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

func getAngle(a coord, pX float64, pY float64, b coord) float64 {
	aTanVertical := math.Atan2(pY-float64(a.Y), pX-float64(a.X))
	aTanToB := math.Atan2(float64(b.Y-a.Y), float64(b.X-a.X))
	return aTanToB - aTanVertical
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
