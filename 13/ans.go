package main

import (
	"bufio"
	"flag"
	"fmt"
	"os"
	"time"

	"github.com/dfontana/adventofcode2019/intcode"
)

const (
	empty   = 0
	wall    = 1
	block   = 2
	hPaddle = 3
	ball    = 4
	timeout = 3 * time.Millisecond // for input
)

type tile struct {
	x  int
	y  int
	id int
}

var ballTileX int
var paddleTileX int
var mode = flag.String("mode", "player", "[bot, player, headless] How to play the game")

func main() {
	flag.Parse()

	tiles := playGame(0)
	count, width, height := getInitialBoardState(tiles)
	fmt.Println("Part 1:", count, "on board", width, "x", height)

	tiles = playGame(2)
	finalScore := printBoard(tiles, width, height)
	fmt.Println("Part 2:", finalScore)
}

func playGame(quarters int) <-chan tile {
	data := intcode.ReadProgram("./input.txt")
	if quarters != 0 {
		data[0] = int64(quarters)
	}
	input, output := intcode.MakeComms()

	go intcode.Run(data, input, output)
	tilesA := listenForTiles(output)
	tilesB := listenForInput(tilesA, input)

	return tilesB
}

func listenForInput(tiles <-chan tile, input chan<- int64) <-chan tile {
	out := make(chan tile)
	go func() {
		timer := time.NewTimer(timeout)
		for stop := false; !stop; {
			select {
			case val, ok := <-tiles:
				timer.Stop()
				if ok {
					out <- val
					timer.Reset(timeout)
				} else {
					stop = true
				}
			case <-timer.C:
				input <- int64(getInput())
			default:
			}
		}
		close(out)
	}()
	return out
}

func listenForTiles(progOut <-chan int64) <-chan tile {
	out := make(chan tile)
	go func() {
		instr := []int{0, 0, 0}
		i := 0
		for val := range progOut {
			instr[i%3] = int(val)
			i++
			if i%3 == 0 {
				// Flush the instr
				if instr[2] == hPaddle {
					paddleTileX = instr[0]
				}
				if instr[2] == ball {
					ballTileX = instr[0]
				}
				out <- tile{instr[0], instr[1], instr[2]}
				i = 0
			}
		}
		close(out)
	}()
	return out
}

func getInput() int {
	if *mode != "player" {
		// Cheaty!
		if ballTileX > paddleTileX {
			paddleTileX++
			return 1
		}
		if ballTileX < paddleTileX {
			paddleTileX--
			return -1
		}
		return 0
	}

	reader := bufio.NewReader(os.Stdin)
	text, _ := reader.ReadString('\n')
	char := text[:len(text)-1]
	switch char {
	case "j":
		return -1
	case "l":
		return 1
	default:
		return 0
	}
}

func printBoard(tiles <-chan tile, width, height int) int {
	// init the board
	var board [][]int
	for row := 0; row <= height; row++ {
		var tileRow []int
		for col := 0; col <= width; col++ {
			tileRow = append(tileRow, empty)
		}
		board = append(board, tileRow)
	}

	// listen for tiles and place them on the board
	currentScore := 0
	for t := range tiles {
		if t.x == -1 && t.y == 0 {
			// Special case: print the score
			currentScore = t.id
		} else {
			// update
			board[t.y][t.x] = t.id
		}

		if *mode == "headless" {
			fmt.Print("\033[0G")
			fmt.Print(currentScore)
			continue
		}

		// clear screen
		fmt.Printf("\033[0;0H")

		// draw
		for _, row := range board {
			for _, col := range row {
				tChar := " "
				switch col {
				case wall:
					tChar = "|"
				case block:
					tChar = "B"
				case hPaddle:
					tChar = "_"
				case ball:
					tChar = "o"
				}
				fmt.Print(tChar)
			}
			fmt.Println()
		}

		// We'll put the score at the bottom
		fmt.Print("Score:", currentScore)
	}
	return currentScore
}

func getInitialBoardState(tiles <-chan tile) (int, int, int) {
	ct, maxX, maxY := 0, 0, 0
	for t := range tiles {
		if t.x > maxX {
			maxX = t.x
		}
		if t.y > maxY {
			maxY = t.y
		}
		if t.id == block {
			ct++
		}
	}
	return ct, maxX, maxY
}
