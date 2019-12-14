package main

import (
	"fmt"

	"github.com/dfontana/adventofcode2019/intcode"
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
	in, out := intcode.MakeComms()

	// TODO getInput chan for the comp to request input, we'll then send it through input.
	// TODO perhaps group these chans into a pointer to a struct that intcode returns + intval for memory[0]
	go intcode.Run(data, in, out)

	tiles := map[string]*tile{"0_0": &tile{false, 0, 0, black}}
	bot := robot{0, 0, 0}

	i := 0
	for stop := false; !stop; {
		select {
		case val, ok := <-out:
			if !ok {
				stop = true
				break
			}

			switch i % 2 {
			case 0:
				// Color instruction
				key := fmt.Sprintf("%d_%d", bot.x, bot.y)
				tiles[key].color = asPaint(int(val))
			case 1:
				// Move instruction
				bot.direction = bot.direction.rotate(asTurn(int(val)))
				if bot.direction.isLateral() {
					bot.x += bot.direction.cartesianDiff()
				} else {
					bot.y += bot.direction.cartesianDiff()
				}
			}

			i++
		case <-getInput:
			key := fmt.Sprintf("%d_%d", bot.x, bot.y)
			val, ok := tiles[key]
			if ok {
				in <- int64(val.color)
			}
		}
	}
}
