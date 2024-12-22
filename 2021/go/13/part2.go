package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Coord struct {
	x, y int
}

func main() {
	file, err := os.Open("../../input/day13.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	coords := make(map[Coord]bool)
	folds := []string{}
	for scanner.Scan() {
		line := scanner.Text()
		if len(line) == 0 {
			continue
		}
		if strings.Contains(line, "fold along") {
			// fold line
			folds = append(folds, line)
		} else {
			// Coord
			split := strings.Split(line, ",")
			x, _ := strconv.Atoi(split[0])
			y, _ := strconv.Atoi(split[1])
			coord := Coord{x: x, y: y}
			coords[coord] = true
		}
	}

	for _, fold := range folds {
		foldLine := strings.Split(strings.Split(fold, " ")[2], "=")
		axis, lineNumStr := foldLine[0], foldLine[1]
		lineNum, _ := strconv.Atoi(lineNumStr)

		newCoords := make(map[Coord]bool)
		for coord := range coords {
			if axis == "y" {
				newCoords[FlipCoordY(coord, lineNum)] = true
			} else {
				newCoords[FlipCoordX(coord, lineNum)] = true
			}
		}
		coords = newCoords
	}

	DisplayPaper(coords)
	fmt.Println(len(coords))

}

func FlipCoordX(coord Coord, line int) Coord {
	// Find distance to the line
	diff := coord.x - line
	// If diff is positive, we are under the line
	if diff > 0 {
		coord.x -= diff * 2
	}
	return coord
}

func FlipCoordY(coord Coord, line int) Coord {
	// Find distance to the line
	diff := coord.y - line
	// If diff is positive, we are under the line
	if diff > 0 {
		coord.y -= diff * 2
	}
	return coord
}

func DisplayPaper(coords map[Coord]bool) {
	var maxX, maxY int
	for coord := range coords {
		if coord.x > maxX {
			maxX = coord.x
		}
		if coord.y > maxY {
			maxY = coord.y
		}
	}

	for y := 0; y <= maxY; y++ {
		for x := 0; x <= maxX; x++ {
			c := Coord{x: x, y: y}
			if _, ok := coords[c]; ok {
				fmt.Print("#")
			} else {
				fmt.Print(".")
			}
		}
		fmt.Println()
	}
}
