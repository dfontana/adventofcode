package main

import (
	"fmt"
	"github.com/dfontana/adventofcode/2019/util"
)

// Layers of pixels
type Layers [][]int

func main() {
	layers := parseLayers("./input.txt", 25, 6)
	minLayer := findFewestZeroes(layers)

	fmt.Println("Part 1:", countDigit(layers[minLayer], 1)*countDigit(layers[minLayer], 2))
	fmt.Print("Part 2:")
	printLayer(decode(layers), 25)
}

func printLayer(layer []int, width int) {
	for i, pixel := range layer {
		if i%width == 0 {
			fmt.Println("")
		}
		switch pixel {
		case 0:
			fmt.Print("-")
		case 1:
			fmt.Print("#")
		case 2:
			fmt.Print(" ")
		}
	}
}

func decode(layers Layers) []int {
	output := make([]int, len(layers[0]))
	for i, pixel := range layers[0] {
		chosenPixel := pixel
		layer := 0
		for chosenPixel == 2 {
			chosenPixel = layers[layer][i]
			layer++
		}
		output[i] = chosenPixel
	}
	return output
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
