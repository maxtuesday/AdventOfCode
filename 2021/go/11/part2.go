package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

const STEP_SIZE = 200

var STEP = 1
var COUNT = 0
var COUNT_MAX = 150

type Coord struct {
	row, col int
}

func main() {
	file, err := os.Open("../../input/day11.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	dumbos := [][]int{}

	for scanner.Scan() {
		nums := strings.Split(scanner.Text(), "")
		row := make([]int, len(nums))
		for i := range nums {
			x, _ := strconv.Atoi(nums[i])
			row[i] = x
		}
		dumbos = append(dumbos, row)
	}
	// Loaded dumbos
	// Simulate Energy Changes
	flashTarget := len(dumbos) * len(dumbos[0])
	for STEP = 1; ; STEP++ {
		flashedCount := SimulateEngeryStep(dumbos)
		if flashedCount == flashTarget {
			fmt.Println("All dumbos flashed on Step:", STEP)
			return
		}
	}
}

func GetSurroundingCoords(row, col, rowLimit, colLimit int) []Coord {
	surrounding := []Coord{
		{row: row - 1, col: col - 1},
		{row: row - 1, col: col},
		{row: row - 1, col: col + 1},
		{row: row, col: col - 1},
		{row: row, col: col + 1},
		{row: row + 1, col: col - 1},
		{row: row + 1, col: col},
		{row: row + 1, col: col + 1},
	}

	safeSurrounding := []Coord{}
	for _, s := range surrounding {
		if s.row >= 0 && s.row <= rowLimit && s.col >= 0 && s.col <= colLimit {
			safeSurrounding = append(safeSurrounding, s)
		}
	}
	return safeSurrounding
}

func FlashRec(dumbos [][]int, curr Coord, flashed map[Coord]bool) {
	// If dumbos[row][col] after increase is greater than 9
	if dumbos[curr.row][curr.col] > 9 {
		// Flashed!
		// Then we need to increase all the surrounding positions energy levels by +1
		// Has this dumbo flashed prior?
		if _, ok := flashed[curr]; ok {
			// Already flashed prior
			// We do not want to increase any neighbors
			return
		}
		// First time flashing, we can increment neighbors
		flashed[curr] = true
		surrounding := GetSurroundingCoords(curr.row, curr.col, len(dumbos)-1, len(dumbos[curr.row])-1)
		for _, coord := range surrounding {
			dumbos[coord.row][coord.col]++
			if dumbos[coord.row][coord.col] > 9 {
				// This neighbor flashed! We should recusively update neighbors
				if _, ok := flashed[coord]; !ok {
					FlashRec(dumbos, Coord{row: coord.row, col: coord.col}, flashed)
				}
			}
		}
	}
}

func SimulateEngeryStep(dumbos [][]int) int {
	flashed := make(map[Coord]bool)
	for row := range dumbos {
		for col := range dumbos {
			// Increase engery by +1
			dumbos[row][col]++
			curr := Coord{row: row, col: col}
			FlashRec(dumbos, curr, flashed)
			// can I look at all 8 other coords?
			// Positions to check:
			// | top left    |     top    | top right    |
			// | right       |      x     | right        |
			// | bottom left |    bottom  | bottom right |
			//
			// Which translates to:
			// | (row-1, col-1) | (row-1, col) | (row-1, col+1) |
			// | (row,   col-1) | (row,   col) | (row  , col+1) |
			// | (row+1, col-1) | (row+1, col) | (row+1, col+1) |
			//
			// If row == 0,                    then do not create the (row - 1) positions
			// If col == 0,                    then do not create the (col - 1) positions
			// If row == len(dumbos) - 1,      then do not create the (row + 1) positions
			// If col == len(dumbos[row]) - 1, then do not create the (col + 1) positions

			// fmt.Printf("Curr: (%d, %d) | Check: %v\n\n", row, col, safeSurrounding)

			// After gathering the coords we can actually work on.

			// Given that this current coordinate has flashed, add it to the `flashed` map
			// For points that we increased the energy for as well, add those to the `flashed` map
		}
	}

	// We have finished a `step` of the energy simulation
	// Iterate through the `flashed` map and set all coords to 0
	for coord := range flashed {
		dumbos[coord.row][coord.col] = 0
	}

	// Also this will give us the count `len(flashed)`
	// Return this value

	return len(flashed)
}

func DumpClean(dumbos [][]int) {
	fmt.Println("--------------------")
	for row := range dumbos {
		for col := range dumbos[row] {
			val := dumbos[row][col]
			if val == 0 || val > 9 {
				fmt.Print("\033[1m0\033[22m")
			} else {
				fmt.Printf("%d", val)
			}
		}
		fmt.Println()
	}
	fmt.Println("--------------------")
}

func Dump(dumbos [][]int) {
	fmt.Println("--------------------")
	for row := range dumbos {
		for col := range dumbos[row] {
			val := dumbos[row][col]
			if val == 0 || val > 9 {
				fmt.Printf(" \033[1m%d\033[22m\t", val)
			} else {
				fmt.Printf(" %d ", val)
			}
		}
		fmt.Println()
	}
	fmt.Println("--------------------")
}

func DumpAndTrack(dumbos [][]int, curr Coord) {
	fmt.Println("--------------------")
	for row := range dumbos {
		for col := range dumbos[row] {
			val := dumbos[row][col]
			var s string
			if val == 0 || val > 9 {
				s = fmt.Sprintf("\033[1m%d\033[22m", val)
			} else {
				s = fmt.Sprintf("%d", val)
			}
			if curr.row == row && curr.col == col {
				fmt.Printf("[ %s ]\t", s)
			} else {
				fmt.Printf(" %s \t", s)
			}
		}
		fmt.Println()
	}
	fmt.Println("--------------------")
}
