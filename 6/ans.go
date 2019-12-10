package main

import (
	"fmt"
	"github.com/dfontana/adventofcode2019/util"
)

func main() {
	edges := parseInput("./input.txt")
	fmt.Println("P1: ", countDirectAndIndirect(edges))
	fmt.Println("P2: ", distBetween("YOU", "SAN", edges))
}

func distBetween(N1 string, N2 string, edges map[string][]string) int {
	path := make(map[string]int)

	// Walk backwards towards COM from N1, storing distances to each parent
	object, dist := findOrbit(N1, edges), 0
	for {
		path[object] = dist
		parent := findOrbit(object, edges)
		if parent == "" {
			break
		}
		object = parent
		dist++
	}

	// Walk backwards towards COM from N2, until first intersect occurs
	object, dist2 := findOrbit(N2, edges), 0
	for {
		pathDist, ok := path[object]
		if ok {
			dist2 += pathDist
			break
		}

		parent := findOrbit(object, edges)
		if parent == "" {
			break
		}
		object = parent
		dist2++
	}
	return dist2
}

func findOrbit(node string, edges map[string][]string) string {
	for parent, neighbors := range edges {
		if inArray(node, neighbors) {
			return parent
		}
	}
	return ""
}

func countDirectAndIndirect(edges map[string][]string) int {
	ct := 0
	node := "COM"
	frontier := []string{node}
	for len(frontier) > 0 {
		node, frontier = frontier[0], frontier[1:]
		for _, neighbor := range edges[node] {
			ct += countNeighbors(neighbor, edges)
			frontier = append(frontier, neighbor)
		}
	}
	return ct + countNeighbors("COM", edges)
}

func countNeighbors(node string, edges map[string][]string) int {
	ct := 0
	frontier := []string{node}
	for len(frontier) > 0 {
		node, frontier = frontier[0], frontier[1:]
		for _, neighbor := range edges[node] {
			ct++
			frontier = append(frontier, neighbor)
		}
	}
	return ct
}

func inArray(item string, list []string) bool {
	for _, l := range list {
		if l == item {
			return true
		}
	}
	return false
}

func parseInput(filename string) map[string][]string {
	lines := util.GetLines(filename)

	edges := make(map[string][]string)
	for _, line := range lines {
		nodes := util.Split(line, "\\)")
		src, dst := nodes[0], nodes[1]
		if edges[src] == nil {
			edges[src] = []string{dst}
		} else {
			edges[src] = append(edges[src], dst)
		}
	}
	return edges
}
