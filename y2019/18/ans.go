package main

import (
	"fmt"
	"unicode"

	"github.com/dfontana/adventofcode/2019/util"
)

const (
	wall   = '#'
	player = '@'
	open   = '.'
)

type pos struct {
	x    int
	y    int
	char rune
}

type node struct {
	pos       pos
	neighbors []edge
}

type edge struct {
	to    node
	doors []rune
	dist  int
}

type path struct {
	pos   pos
	isKey bool
	doors []rune
	steps int
	prior *path
}

func main() {
	tunnels := parseInput("./input.txt")
	trimmed := truncateMaze(tunnels)
	pPos := findPlayer(trimmed)
	if pPos.x == -1 {
		panic("Couldn't find player")
	}
	printMaze(trimmed)

	root := buildGraph(trimmed, pPos)
	fmt.Println(root)

	// Build graph from root where each edge is between keys with the number of spaces
	// seperating them (densely connected) and the list of doors blocking them. Then
	// search the graph starting at root, goal of visiting every edge. Edges only unlock
	// when the all doors are unlocked. Djikstras, but each edge has dependencies that
	// first must be satisfied?
}

func buildGraph(maze [][]rune, start pos) node {
	frontier := []path{path{start, true, make([]rune, 0), 0, nil}}
	seen := map[pos]bool{start: true}
	graph := map[pos]node{start: node{start, make([]edge, 0)}}

	var item path
	for len(frontier) != 0 {
		item, frontier = frontier[0], frontier[1:]

		seen[item.pos] = true

		// if you are a door, note yourself to be added to the next items doors.
		char := maze[item.pos.y][item.pos.x]
		isDoor := unicode.IsUpper(char)
		doors := item.doors
		if isDoor {
			doors = append(doors, char)
		}

		// If you are a key:
		//		1 check if a node exists in the graph at this position. If not, make item & hold onto the doors & steps
		//		2 unwind your path until you hit another node (isKey) or starting
		//      3 add an edge on the node, from me to unwound node with steps as dist and doors as doors.
		isKey := unicode.IsLower(char)
		if isKey {
			n, ok := graph[item.pos]
			if !ok {
				n = node{item.pos, make([]edge, 0)}
				graph[item.pos] = n
			}

			ptr := item.prior
			for ptr != nil && !ptr.isKey {
				ptr = ptr.prior
			}

			if ptr == nil {
				panic("Node couldnt find the edge it came from")
			}

			toNode, ok := graph[ptr.pos]
			if !ok {
				panic("Encountered toNode we havent yet seen... how")
			}

			newEdge := edge{toNode, item.doors, item.steps}
			n.neighbors = append(n.neighbors, newEdge)
		}

		// Then check all your neighbors, and add them if you havent been visited
		neighbors := getNeighbors(item.pos, maze)
		for _, nPos := range neighbors {
			if seen[nPos] {
				continue
			}
			// Otherwise push a path for each node
			frontier = append(frontier, path{nPos, isKey, doors, item.steps + 1, &item})
		}
	}
	return graph[start]
}

func getNeighbors(item pos, maze [][]rune) []pos {
	var nPoses []pos
	// TODO check 4 diagonals for open space
	// For any that are push a new path with x, y, and the rune type there
	return nPoses
}

func printMaze(maze [][]rune) {
	for _, r := range maze {
		for _, c := range r {
			fmt.Print(string(c))
		}
		fmt.Println()
	}
}

func truncateMaze(tunnels [][]rune) [][]rune {
	// Iterate through maze. Look for all places with 3 # in all directions.
	// If true set self to #. Continue till an iteration finishes without finding any.
	for {
		totalRows, totalCols := len(tunnels), len(tunnels[0])

		foundOne := false
		for r, row := range tunnels {
			for c, col := range row {
				if col != '.' {
					continue
				}

				if r == 0 || r == totalRows-1 || c == 0 || c == totalCols-1 {
					continue
				}

				// We're within bounds and can safely check
				totalWalls := 0
				if tunnels[r-1][c] == wall {
					totalWalls++
				}
				if tunnels[r+1][c] == wall {
					totalWalls++
				}
				if tunnels[r][c+1] == wall {
					totalWalls++
				}
				if tunnels[r][c-1] == wall {
					totalWalls++
				}
				if totalWalls >= 3 {
					foundOne = true
					tunnels[r][c] = wall
				}
			}
		}

		if !foundOne {
			break
		}
	}

	return tunnels
}

func findPlayer(tunnels [][]rune) pos {
	for r, row := range tunnels {
		for c, col := range row {
			if col == player {
				return pos{c, r, '@'}
			}
		}
	}
	return pos{-1, -1, ' '}
}

func parseInput(filename string) [][]rune {
	lines := util.GetLines(filename)

	var tunnels [][]rune
	for _, line := range lines {
		var row []rune
		for _, char := range line {
			row = append(row, char)
		}
		tunnels = append(tunnels, row)
	}
	return tunnels
}
