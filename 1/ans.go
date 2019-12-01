package main

import (
	"fmt"
	"log"
	"math"
	"os"
	"strconv"

	"github.com/dfontana/adventofcode2019/util"
)

// fuelAdder funcs take in an integer and produces one after computing
// any fuel requirements
type fuelAdder func(int) int

func main() {
	var input []int

	if len(os.Args) != 1 {
		val, err := strconv.Atoi(os.Args[1])
		if err != nil {
			log.Fatal(err)
		}
		input = []int{val}
	} else {
		b, err := util.OpenFile("./input.txt")
		if err != nil {
			log.Fatal(err)
		}

		val, err := util.ReadInts(b)
		if err != nil {
			log.Fatal(err)
		}
		input = val
	}

	fmt.Println("Just Module Mass: ", getTotalFuel(input, withModuleMass))
	fmt.Println("Module And Fuel Mass: ", getTotalFuel(input, withFuelMass))
}

func getTotalFuel(moduleMasses []int, adder fuelAdder) int {
	sum := 0
	for _, mass := range moduleMasses {
		sum += adder(mass)
	}
	return sum
}

func withFuelMass(mass int) int {
	fuelMass := fuelNeeded(mass)
	if fuelMass <= 0 {
		return 0
	}
	return fuelMass + withFuelMass(fuelMass)
}

func withModuleMass(mass int) int {
	return fuelNeeded(mass)
}

func fuelNeeded(mass int) int {
	return int(math.Floor(float64(mass)/3.0)) - 2
}
