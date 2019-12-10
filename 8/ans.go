package main

import (
	"fmt"
	"github.com/dfontana/adventofcode2019/util"
)

// Layers of pixels
type Layers [][]int

func main() {
	layers := parseLayers("./input.txt", 25, 6)
	minLayer := findFewestZeroes(layers)

	fmt.Println("Part 1:", countDigit(layers[minLayer], 1)*countDigit(layers[minLayer], 2))
}

func countDigit(layer []int, digit int) int {
	ct := 0
	for _, val := range layer {
		if val == digit {
			ct++
		}
	}
	return ct
}

func findFewestZeroes(layers Layers) int {
	minLayer := 0
	minZeroes := 1000000
	for i, layer := range layers {
		ct := countDigit(layer, 0)
		if ct < minZeroes {
			minLayer = i
			minZeroes = ct
		}
	}
	return minLayer
}

func parseLayers(file string, width int, height int) [][]int {
	lines := util.GetLines(file)
	input := lines[0]
	layerSize := width * height
	totalLayers := len(input) / layerSize

	layers := make([][]int, totalLayers)

	col, row, layer := 0, 0, -1
	for i, num := range input {
		col = i % (width)
		if row+col == 0 {
			layer++
			layers[layer] = make([]int, layerSize)
		}

		layers[layer][(row*width)+col] = util.ToInt(string(num))

		if col == width-1 {
			row++
			row = row % height
		}
	}

	return layers
}
