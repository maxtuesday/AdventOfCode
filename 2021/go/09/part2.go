package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
)

type Coord struct {
	x, y int
	// visited bool
}

var retVisited = []Coord{}

var count = 0

func main() {
	file, err := os.Open("../../input/day09.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	heightMap := [][]int{}
	for scanner.Scan() {
		lineNums := []int{}
		for _, s := range strings.Split(scanner.Text(), "") {
			n, _ := strconv.Atoi(s)
			lineNums = append(lineNums, n)
		}
		heightMap = append(heightMap, lineNums)
	}

	basins := []int{}
	for row := range heightMap {
		for col := range heightMap[row] {
			var left, right, up, down int
			level := heightMap[row][col]

			if row == 0 {
				up = level + 1
			} else {
				up = heightMap[row-1][col]
			}

			if row+1 == len(heightMap) {
				down = level + 1
			} else {
				down = heightMap[row+1][col]
			}

			if col == 0 {
				left = level + 1
			} else {
				left = heightMap[row][col-1]
			}

			if col+1 == len(heightMap[row]) {
				right = level + 1
			} else {
				right = heightMap[row][col+1]
			}

			if level < left &&
				level < right &&
				level < up &&
				level < down {
				// This is a lowest point to consider
				// Search of other points around to find the basin sum
				curr := Coord{x: col, y: row}
				visited := make(map[Coord]int)
				exploreBasin(heightMap, visited, curr)
				basins = append(basins, len(visited))
			}
		}
	}

	sort.Ints(basins)

	l := len(basins)
	maxBasins := basins[l-3 : l]
	product := 1
	for _, b := range maxBasins {
		product *= b
	}
	fmt.Println("Result:", product)
}

// Visited as a mao[Coord]int?
func exploreBasin(source [][]int, visited map[Coord]int, curr Coord) {
	if source[curr.y][curr.x] == 9 {
		return
	}

	// Given the current coordinate
	// Find the next available spots to search
	xLimit := len(source[0])
	yLimit := len(source)

	nextCoords := []Coord{}
	if curr.y >= 0 && curr.y < yLimit-1 {
		nextCoords = append(nextCoords, Coord{x: curr.x, y: curr.y + 1})
	}
	if curr.y > 0 && curr.y <= yLimit-1 {
		nextCoords = append(nextCoords, Coord{x: curr.x, y: curr.y - 1})
	}
	if curr.x >= 0 && curr.x < xLimit-1 {
		nextCoords = append(nextCoords, Coord{x: curr.x + 1, y: curr.y})
	}
	if curr.x > 0 && curr.x <= xLimit-1 {
		nextCoords = append(nextCoords, Coord{x: curr.x - 1, y: curr.y})
	}

	// We have the next coordinate to look at based on the current coordinate
	// However, there are coordinates which we have already visited
	// Exclude those coordinate from the list to continue to look at
	unvisitedCoords := []Coord{}
	for _, n := range nextCoords {
		if _, ok := visited[n]; !ok {
			unvisitedCoords = append(unvisitedCoords, n)
		}
	}

	visited[curr]++
	for _, u := range unvisitedCoords {
		exploreBasin(source, visited, u)
	}
}
