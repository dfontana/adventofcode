package main

import (
	"fmt"
	"math"
	"regexp"

	"github.com/dfontana/adventofcode2019/util"
)

// Vector3 in 3D space
type Vector3 struct {
	X int
	Y int
	Z int
}

func (v *Vector3) add(p Vector3) {
	v.X = p.X + v.X
	v.Y = p.Y + v.Y
	v.Z = p.Z + v.Z
}

func (v Vector3) absSum() int {
	return int(math.Abs(float64(v.X)) + math.Abs(float64(v.Y)) + math.Abs(float64(v.Z)))
}

// Moon with Position and Velocity in 3D space
type Moon struct {
	P Vector3
	V Vector3
}

func (m *Moon) accel() {
	m.P.add(m.V)
}

func (m *Moon) gravX(other *Moon) {
	amount := -1
	if m.P.X == other.P.X {
		return
	}
	if m.P.X < other.P.X {
		amount = 1
	}
	m.V.X = m.V.X + amount
	other.V.X = other.V.X - amount
}

func (m *Moon) gravY(other *Moon) {
	amount := -1
	if m.P.Y == other.P.Y {
		return
	}
	if m.P.Y < other.P.Y {
		amount = 1
	}
	m.V.Y = m.V.Y + amount
	other.V.Y = other.V.Y - amount
}

func (m *Moon) gravZ(other *Moon) {
	amount := -1
	if m.P.Z == other.P.Z {
		return
	}
	if m.P.Z < other.P.Z {
		amount = 1
	}
	m.V.Z = m.V.Z + amount
	other.V.Z = other.V.Z - amount
}

func (m Moon) potential() int {
	return m.P.absSum()
}

func (m Moon) kinetic() int {
	return m.V.absSum()
}

// Moons collection
type Moons map[int]*Moon

func (ms Moons) applyGravity() {
	ct := len(ms)
	for i := 0; i < ct; i++ {
		for j := i + 1; j < ct; j++ {
			moon := ms[i]
			moon2 := ms[j]
			moon.gravX(moon2)
			moon.gravY(moon2)
			moon.gravZ(moon2)
		}
	}
}

func (ms Moons) applyVelocity() {
	for _, moon := range ms {
		moon.accel()
	}
}

func (ms Moons) totalEnergy() int {
	tot := 0
	for _, moon := range ms {
		tot += moon.kinetic() * moon.potential()
	}
	return tot
}

func (ms Moons) print() {
	for i, m := range ms {
		fmt.Println(i, ":", *m)
	}
}

func main() {
	moons := parseInput("./input.txt")
	time := 1000

	simulate(moons, time)
	moons.print()
	fmt.Println("Total Enegery", moons.totalEnergy())
}

func simulate(moons Moons, time int) {
	for i := 0; i < time; i++ {
		moons.applyGravity()
		moons.applyVelocity()
	}
}

func parseInput(filename string) Moons {
	lines := util.GetLines(filename)
	moons := make(Moons)
	r := regexp.MustCompile(`-*[0-9]+`)
	for i, line := range lines {
		matches := r.FindAllString(line, -1)
		pos := Vector3{
			X: util.ToInt(matches[0]),
			Y: util.ToInt(matches[1]),
			Z: util.ToInt(matches[2]),
		}
		moons[i] = &Moon{
			P: pos,
			V: Vector3{0, 0, 0},
		}
	}
	return moons
}
