package main

import (
	"fmt"
	"github.com/dfontana/adventofcode2019/util"
)

func main() {
	edges := parseInput("./input.txt")
	//edges := parseInput("./test.txt")
	fmt.Println("P1: ", countDirectAndIndirect(edges))
}

func countDirectAndIndirect(edges map[string][]string) int {
	ct := 0
	node := "COM"
	frontier := []string{node}
	var visited []string
	for len(frontier) > 0 {
		node, frontier = frontier[0], frontier[1:]
		if inArray(node, visited) {
			continue
		}
		visited = append(visited, node)
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
	var visited []string
	for len(frontier) > 0 {
		node, frontier = frontier[0], frontier[1:]
		if inArray(node, visited) {
			continue
		}
		visited = append(visited, node)
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
