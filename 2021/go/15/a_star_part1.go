package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
	"strings"
)

const INPUT_FILE = "../../input/day15.txt"

var MAX_X int
var MAX_Y int
var GRAPH [][]int

type Pos struct {
	x, y int
}

func (p Pos) Neighbors() []Pos {
	coords := []Pos{}
	down := Pos{x: p.x, y: p.y + 1}
	up := Pos{x: p.x, y: p.y - 1}
	right := Pos{x: p.x + 1, y: p.y}
	left := Pos{x: p.x - 1, y: p.y}

	if p.x > 0 {
		coords = append(coords, left)
	}
	if p.x < MAX_X {
		coords = append(coords, right)
	}

	if p.y > 0 {
		coords = append(coords, up)
	}
	if p.y < MAX_X {
		coords = append(coords, down)
	}

	return coords
}

type Path struct {
	path []Pos
}

const COUNT_MAX = 20

var COUNT = 0

func LoadData(filename string) [][]int {
	file, err := os.Open(filename)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	riskMap := [][]int{}
	for scanner.Scan() {
		row := []int{}
		for _, s := range strings.Split(scanner.Text(), "") {
			n, _ := strconv.Atoi(s)
			row = append(row, n)
		}
		riskMap = append(riskMap, row)
	}
	return riskMap
}

func DumpMap(riskMap [][]int) {
	for row := range riskMap {
		for col := range riskMap[row] {
			fmt.Printf("%d", riskMap[row][col])
		}
		fmt.Println()
	}
}

func DumpMapWithPath(riskMap [][]int, visited map[Pos]bool) {
	for row := range riskMap {
		for col := range riskMap[row] {
			coord := Pos{x: col, y: row}
			if _, ok := visited[coord]; ok {
				fmt.Printf("\033[1;32m%d\033[0m", riskMap[row][col])
			} else {
				fmt.Printf("%d", riskMap[row][col])
			}
		}
		fmt.Println()
	}
}

func Distance(a, b Pos) float64 {
	dy := float64(b.y - a.y)
	dx := float64(b.x - a.x)

	return math.Sqrt(math.Pow(dy, 2) + math.Pow(dx, 2))
}

func H(pos Pos) float64 {
	end := Pos{x: MAX_X, y: MAX_Y}
	risk := GRAPH[pos.y][pos.x]
	dist := Distance(pos, end)
	return float64(risk) + dist
}

func ReconstructPath(cameFrom map[Pos]Pos, current Pos) []Pos {
	path := []Pos{current}
	for {
		if _, ok := cameFrom[current]; !ok {
			break
		}
		current = cameFrom[current]
		path = append(path, current)
	}
	return path
}

func A_Star(start Pos, goal Pos) []Pos {
	openSet := make(map[Pos]bool)
	openSet[start] = true

	cameFrom := make(map[Pos]Pos)

	gScore := make(map[Pos]float64)
	gScore[start] = 0

	fScore := make(map[Pos]float64)
	fScore[start] = H(start)

	for len(openSet) != 0 {
		var current Pos
		minScore := math.MaxFloat64
		for n := range openSet {
			if fScore[n] < minScore {
				minScore = fScore[n]
				current = n
			}
		}
		// fmt.Println("Current:", current)

		if current == goal {
			fmt.Println("!!! Found Goal !!!")
			return ReconstructPath(cameFrom, current)
		}

		delete(openSet, current)

		for _, neighbor := range current.Neighbors() {
			tentativeScore := gScore[current] + 1 + float64(GRAPH[neighbor.y][neighbor.x])
			// fmt.Println("Neighbor:", neighbor)

			if score, ok := gScore[neighbor]; ok {
				if tentativeScore < score {
					// fmt.Println("Lower score:", neighbor)
					cameFrom[neighbor] = current
					gScore[neighbor] = tentativeScore
					fScore[neighbor] = H(neighbor)
					openSet[neighbor] = true
				}
			} else {
				// fmt.Println("Path is better than previous one:", neighbor)
				cameFrom[neighbor] = current
				gScore[neighbor] = tentativeScore
				fScore[neighbor] = tentativeScore + H(neighbor)
				openSet[neighbor] = true
			}
		}

		// fmt.Println("Open Set:", openSet)
		// fmt.Println()
		// DumpMapWithPath(GRAPH, openSet)
		// fmt.Println()

	}

	return []Pos{}
}

func main() {
	GRAPH = LoadData(INPUT_FILE)
	MAX_Y = len(GRAPH) - 1
	MAX_X = len(GRAPH[0]) - 1

	start := Pos{x: 0, y: 0}
	goal := Pos{x: MAX_X, y: MAX_Y}
	path := A_Star(start, goal)

	fmt.Println("END")
	fmt.Println("Path:", path)
	fmt.Println()

	riskSum := -GRAPH[start.y][start.x]
	visited := make(map[Pos]bool)
	for _, p := range path {
		riskSum += GRAPH[p.y][p.x]
		visited[p] = true
	}

	DumpMapWithPath(GRAPH, visited)

	fmt.Println("Risk Sum:", riskSum)

}
