package main

import (
	"fmt"
	"math"
	"regexp"

	"github.com/dfontana/adventofcode/2019/util"
)

// Vector3 in 3D space
type Vector3 struct {
	X int
	Y int
	Z int
}

func (v Vector3) get(dim rune) int {
	switch dim {
	case 'x':
		return v.X
	case 'y':
		return v.Y
	default:
		return v.Z
	}
}

func (v *Vector3) set(dim rune, val int) {
	switch dim {
	case 'x':
		v.X = val
	case 'y':
		v.Y = val
	case 'z':
		v.Z = val
	}
}

// Moon with Position and Velocity in 3D space
type Moon struct {
	P Vector3
	V Vector3
}

func (m *Moon) accel() {
	m.P.X += m.V.X
	m.P.Y += m.V.Y
	m.P.Z += m.V.Z
}

func (m *Moon) grav(dim rune, other *Moon) {
	if m.P.get(dim) == other.P.get(dim) {
		return
	}
	amount := -1
	if m.P.get(dim) < other.P.get(dim) {
		amount = 1
	}
	m.V.set(dim, m.V.get(dim)+amount)
	other.V.set(dim, other.V.get(dim)-amount)
}

// Moons collection
type Moons map[int]*Moon

func (ms Moons) step() {
	ct := len(ms)
	// Apply Gravity
	for i := 0; i < ct; i++ {
		for j := i + 1; j < ct; j++ {
			ms[i].grav('x', ms[j])
			ms[i].grav('y', ms[j])
			ms[i].grav('z', ms[j])
		}
	}

	// Then Velocity
	for _, moon := range ms {
		moon.accel()
	}
}

func (ms Moons) totalEnergy() int {
	tot := 0
	for _, m := range ms {
		kin := math.Abs(float64(m.V.X)) + math.Abs(float64(m.V.Y)) + math.Abs(float64(m.V.Z))
		pot := math.Abs(float64(m.P.X)) + math.Abs(float64(m.P.Y)) + math.Abs(float64(m.P.Z))
		tot += int(kin * pot)
	}
	return tot
}

func lcm(a, b int) int {
	x, y := a, b
	for y != 0 {
		x, y = y, x%y
	}
	return a / x * b
}

func main() {
	moons := parseInput("./input.txt")
	for i := 0; i < 1000; i++ {
		moons.step()
	}
	fmt.Println("P1 Total Enegery", moons.totalEnergy())

	// All 3 dims do not affect each other. So find the shortest time it takes for each to
	// independently repeat the first state (period). Then find the least common multiple of all
	// 3 periods - that's the soonest all 3 will align.
	moonStart := parseInput("./input.txt")
	moonSteps := parseInput("./input.txt")

	periodX, periodY, periodZ := 0, 0, 0
	for step := 1; periodX == 0 || periodY == 0 || periodZ == 0; step++ {
		moonSteps.step()

		foundX, foundY, foundZ := true, true, true
		for i, moon := range moonSteps {
			if moon.P.X != moonStart[i].P.X || moon.V.X != moonStart[i].V.X {
				foundX = false
			}
			if moon.P.Y != moonStart[i].P.Y || moon.V.Y != moonStart[i].V.Y {
				foundY = false
			}
			if moon.P.Z != moonStart[i].P.Z || moon.V.Z != moonStart[i].V.Z {
				foundZ = false
			}
		}

		if foundX && periodX == 0 {
			periodX = step
		}
		if foundY && periodY == 0 {
			periodY = step
		}
		if foundZ && periodZ == 0 {
			periodZ = step
		}
	}
	fmt.Println("P2", lcm(lcm(periodX, periodY), periodZ))
}

func parseInput(filename string) Moons {
	moons := make(Moons)
	r := regexp.MustCompile(`-*[0-9]+`)
	for i, line := range util.GetLines(filename) {
		matches := r.FindAllString(line, -1)
		pos := Vector3{util.ToInt(matches[0]), util.ToInt(matches[1]), util.ToInt(matches[2])}
		moons[i] = &Moon{pos, Vector3{0, 0, 0}}
	}
	return moons
}
