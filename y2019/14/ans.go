package main

import (
	"fmt"
	"regexp"
	"strings"

	"github.com/dfontana/adventofcode/2019/util"
)

type chemical struct {
	amount int
	name   string
}

type formula struct {
	chemical
	inputs []chemical
}

func main() {
	formulas := parseInput("./input.txt")
	orePerFuel := synthesize("FUEL", 1, formulas)["ORE"]
	fmt.Println("Part 1: 1 Fuel costs", orePerFuel, "ORE")

	availableOre := 1_000_000_000_000
	testAmount, finalAmount := 0, 0
	bestGuess := availableOre / orePerFuel
	for {
		oreNeeded := synthesize("FUEL", testAmount, formulas)["ORE"]
		if oreNeeded > availableOre {
			testAmount -= bestGuess
		} else if oreNeeded <= availableOre {
			finalAmount = testAmount
			testAmount += bestGuess
		}

		if bestGuess > 1 {
			bestGuess /= 2
		} else {
			// Search has converged
			break
		}
	}
	fmt.Println("Part 2: We can make at most", finalAmount, "fuel")
}

func synthesize(name string, amount int, formulas map[string]formula) map[string]int {
	reduction := map[string]int{name: amount}

	for done := false; !done; {
		done = true
		for name, amount := range reduction {
			if amount <= 0 {
				continue
			}
			if form, ok := formulas[name]; ok {
				done = false
				batches := (form.amount + amount - 1) / form.amount
				reduction[name] -= batches * form.amount
				for _, input := range form.inputs {
					reduction[input.name] += batches * input.amount
				}
			}
		}
	}

	return reduction
}

func parseInput(filename string) map[string]formula {
	lines := util.GetLines(filename)
	reg := regexp.MustCompile("([0-9]+) ([A-Z]+)")

	formulas := make(map[string]formula)
	for _, line := range lines {
		tokens := reg.FindAllString(line, -1)
		ct := len(tokens)
		var chems []chemical
		var chem chemical
		for i, tok := range tokens {
			items := strings.Split(tok, " ")
			chem = chemical{util.ToInt(items[0]), items[1]}
			if i != ct-1 {
				chems = append(chems, chem)
			}
		}
		formulas[chem.name] = formula{chem, chems}
	}

	return formulas
}
