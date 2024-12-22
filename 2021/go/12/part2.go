package main

import (
	"fmt"
	"log"
	"os"
	"strings"
)

const START = "start"
const END = "end"

const COUNT_MAX = 50

var COUNT = 0

var paths = [][]string{}

type Visited struct {
	v map[string]int
}

func main() {
	content, err := os.ReadFile("../../input/day12.txt")
	if err != nil {
		log.Fatal(err)
	}

	connections := strings.Split(string(content), "\n")
	connections = connections[:len(connections)-1]

	graph := make(map[string][]string)

	for _, connection := range connections {
		// fmt.Println(connection)
		caves := strings.Split(connection, "-")
		graph[caves[0]] = append(graph[caves[0]], caves[1])
		graph[caves[1]] = append(graph[caves[1]], caves[0])
	}

	Dump(graph)

	fmt.Println()

	for _, start := range graph[START] {
		ExplorePath(start, graph, []string{})
	}

	fmt.Println("Num Paths:", len(paths))
}

func ExplorePath(curr string, graph map[string][]string, path []string) []string {

	path = append(path, curr)

	// Is curr `end`
	if curr == END {
		return path
	}

	// Find which spots are left to explore
	next := []string{}
	possible := graph[curr]
	visited := make(map[string]int)
	for _, v := range path {
		visited[v]++
	}

	// Visit only ONE small cave once
	// Find if there are any
	smallCaveVisitedTwice := true
	for cave, val := range visited {
		if strings.ToLower(cave) == cave && val == 2 {
			smallCaveVisitedTwice = false
		}
	}

	for _, p := range possible {
		if p != START {
			if c, ok := visited[p]; ok {
				// Found
				// Check if it is a small or large cave
				if strings.ToLower(p) != p {
					// Large cave, explore as much as we want
					next = append(next, p)
				} else {
					// Small cave
					// How many times have we visited it?
					if c < 2 && smallCaveVisitedTwice {
						next = append(next, p)
					}
				}
			} else {
				// Not found, we can explore it
				next = append(next, p)
			}
		}
	}

	if len(next) == 0 {
		return nil
	}

	for _, n := range next {
		finishedPath := ExplorePath(n, graph, path)
		if finishedPath != nil {
			paths = append(paths, finishedPath)
		}
	}

	return nil
}

func Dump(graph map[string][]string) {
	for k, v := range graph {
		fmt.Printf("%s\t->\t%v\n", k, v)
	}
}
