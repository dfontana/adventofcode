package main

import (
	"fmt"

	"github.com/dfontana/adventofcode/2019/intcode"
)

const (
	white paint = 1
	black paint = 0

	right turn = 1
	left  turn = 0

	north cardinal = 0
	south cardinal = 1
	east  cardinal = 2
	west  cardinal = 3
)

var (
	toLeft = map[cardinal]cardinal{
		north: west,
		west:  south,
		south: east,
		east:  north,
	}
	toRight = map[cardinal]cardinal{
		north: east,
		east:  south,
		south: west,
		west:  north,
	}
)

type paint int
type turn int
type cardinal int

func (c cardinal) cartesianDiff() int {
	if c == south || c == west {
		return -1
	}
	return 1
}

func (c cardinal) rotate(dir turn) cardinal {
	if dir == left {
		return toLeft[c]
	}
	return toRight[c]
}

func (c cardinal) isLateral() bool {
	return (c == west || c == east)
}

type tile struct {
	wasPainted bool
	x          int
	y          int
	color      paint
}

type robot struct {
	direction cardinal
	x         int
	y         int
}

func asPaint(code int) paint {
	switch code {
	case 0:
		return black
	case 1:
		return white
	default:
		panic("Unknown color code")
	}
}

func asTurn(code int) turn {
	switch code {
	case 0:
		return left
	case 1:
		return right
	default:
		panic("Unknown turn code")
	}
}

func main() {
	data := intcode.ReadProgram("./input.txt")

	// Part 1 - Count tiles it paints
	conf := intcode.Config().SendRequest().SendDone()
	go intcode.Run(data, conf)
	tiles := map[string]*tile{"0_0": &tile{false, 0, 0, black}}
	runBot(
		tiles,
		conf.Done,
		conf.Request,
		conf.Output,
		conf.Input,
	)

	ct := 0
	for _, v := range tiles {
		if v.wasPainted {
			ct++
		}
	}
	fmt.Println("Part 1", ct)

	// Part 2 - Print what it spells when starting on white
	conf = intcode.Config().SendRequest().SendDone()
	go intcode.Run(data, conf)
	tiles = map[string]*tile{"0_0": &tile{false, 0, 0, white}}
	runBot(
		tiles,
		conf.Done,
		conf.Request,
		conf.Output,
		conf.Input,
	)

	fmt.Println("Part 2 (upside down)")
	printTiles(tiles)
}

func printTiles(tiles map[string]*tile) {
	// We need to normalize everyone against a new origin.
	// So find the most negative x and positive y
	minX, minY := 0, 0
	maxX, maxY := 0, 0
	for _, tile := range tiles {
		if tile.x < minX {
			minX = tile.x
		}
		if tile.x > maxX {
			maxX = tile.x
		}
		if tile.y > maxY {
			maxY = tile.y
		}
		if tile.y < minY {
			minY = tile.y
		}
	}

	// Init the hull with correct size
	width := maxX - minX
	height := maxY - minY
	var hull [][]paint
	for r := 0; r <= height; r++ {
		row := make([]paint, width+1)
		for c := 0; c <= width; c++ {
			row[c] = black
		}
		hull = append(hull, row)
	}

	// Write the tiles onto the board with adjusted origin
	for _, tile := range tiles {
		hull[tile.y-minY][tile.x+minX] = tile.color
	}

	// And print the reshaped hull
	for _, row := range hull {
		for _, col := range row {
			char := " "
			if col == white {
				char = ":"
			}
			fmt.Print(char)
		}
		fmt.Println()
	}
}

func runBot(tiles map[string]*tile, done, request <-chan bool, output <-chan int64, input chan<- int64) {
	bot := robot{0, 0, 0}
	i := 0
	for stop := false; !stop; {
		select {
		case <-done:
			stop = true
		case val := <-output:
			switch i % 2 {
			case 0:
				// Color instruction
				key := fmt.Sprintf("%d_%d", bot.x, bot.y)
				tiles[key].color = asPaint(int(val))
				tiles[key].wasPainted = true
			case 1:
				// Move instruction
				bot.direction = bot.direction.rotate(asTurn(int(val)))
				if bot.direction.isLateral() {
					bot.x += bot.direction.cartesianDiff()
				} else {
					bot.y += bot.direction.cartesianDiff()
				}
				key := fmt.Sprintf("%d_%d", bot.x, bot.y)
				if _, ok := tiles[key]; !ok {
					tiles[key] = &tile{false, bot.x, bot.y, black}
				}
			}

			i++
		case <-request:
			key := fmt.Sprintf("%d_%d", bot.x, bot.y)
			val, ok := tiles[key]
			if !ok {
				panic("You didn't init the tile the bot is on")
			}
			input <- int64(val.color)
		}
	}
}
